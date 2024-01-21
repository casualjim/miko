use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use axum::{routing::get, Router};
    use axum_session::{SessionConfig, SessionLayer, SessionStore};
    use axum_session_auth::{AuthConfig, AuthSessionLayer, SessionPgPool};
    use leptos::*;
    use leptos_axum::LeptosRoutes;
    use miko::{
      app::{state::AppState, *},
      fileserv::file_and_error_handler,
      models::User,
    };
    use sqlx::PgPool;
    use tracing::Subscriber;
    use tracing_subscriber::{
      fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, registry::LookupSpan, EnvFilter, Layer,
    };
    use uuid::Uuid;

  }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::registry()
    .with(build_loglevel_filter_layer("info,miko=debug"))
    .with(build_logger_text())
    .init();

  let database_url = ::dotenvy::var("MIKO_DATABASE_URL").expect("MIKO_DATABASE_URL must be set");
  tracing::info!("Connecting to database with {}.", &database_url);
  // let pool = PgPool::connect(&database_url).await?;

  let client_id =
    ::dotenvy::var("ZITADEL_CLIENT_ID").expect("expected $ZITADEL_CLIENT_ID to be set");

  // Setting get_configuration(None) means we'll be using cargo-leptos's env values
  // For deployment these variables are:
  // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
  // Alternately a file can be specified such as Some("Cargo.toml")
  // The file would need to be included with the executable when moved to deployment
  let conf = get_configuration(None).await.unwrap();
  let leptos_options = conf.leptos_options;
  let addr = leptos_options.site_addr;

  let state = AppState::new(database_url, client_id, leptos_options)
    .await?
    .with_redirect_url("http://localhost:3000/oauth/finish");

  let session_config = SessionConfig::default().with_table_name("axum_sessions");
  let auth_config = AuthConfig::<Uuid>::default();
  let session_store =
    SessionStore::<SessionPgPool>::new(Some(state.pool.clone().into()), session_config)
      .await
      .unwrap();

  // build our application with a route
  let app = Router::new()
    .route("/oauth/start", get(handlers::start_login))
    .route("/oauth/finish", get(handlers::get_access_token))
    .nest("/openai/v1", miko::server::localai::routes(state.clone()))
    .nest("/api/v1", miko::server::routes(state.clone()))
    .route(
      "/bff/*fn_name",
      get(handlers::server_fn_handler).post(handlers::server_fn_handler),
    )
    .leptos_routes_with_handler(state.routes.clone(), get(handlers::leptos_routes_handler))
    .fallback(file_and_error_handler)
    .layer(
      AuthSessionLayer::<User, Uuid, SessionPgPool, PgPool>::new(Some(state.pool.clone()))
        .with_config(auth_config),
    )
    .layer(SessionLayer::new(session_store))
    .with_state(state);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  tracing::info!("listening on http://{}", &addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
  Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
  // no client-side main function
  // unless we want this to work with e.g., Trunk for a purely client-side app
  // see lib.rs for hydration function instead
}

#[cfg(feature = "ssr")]
fn build_logger_text<S>() -> Box<dyn Layer<S> + Send + Sync + 'static>
where
  S: Subscriber + for<'a> LookupSpan<'a>,
{
  if cfg!(debug_assertions) {
    Box::new(
      tracing_subscriber::fmt::layer()
        .pretty()
        .with_line_number(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_timer(tracing_subscriber::fmt::time::time()),
    )
  } else {
    Box::new(
      tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(true)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_timer(tracing_subscriber::fmt::time::time()),
    )
  }
}

#[cfg(feature = "ssr")]
fn build_loglevel_filter_layer<S: Into<String>>(default_log: S) -> EnvFilter {
  // filter what is output on log (fmt)
  std::env::set_var(
    "RUST_LOG",
    std::env::var("KATNIP_LOG_LEVEL")
      .or_else(|_| std::env::var("RUST_LOG"))
      .unwrap_or_else(|_| default_log.into()),
  );
  EnvFilter::from_default_env()
}
