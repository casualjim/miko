#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
  use uuid::Uuid;

  simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

  let database_url = ::dotenvy::var("MIKO_DATABASE_URL").expect("MIKO_DATABASE_URL must be set");
  log::info!("Connecting to database with {}.", &database_url);
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
    .route(
      "/bff/*fn_name",
      get(handlers::server_fn_handler).post(handlers::server_fn_handler),
    )
    .leptos_routes_with_handler(state.routes.clone(), get(handlers::leptos_routes_handler))
    .route("/oauth/start", get(handlers::start_login))
    .route("/oauth/finish", get(handlers::get_access_token))
    .fallback(file_and_error_handler)
    .layer(
      AuthSessionLayer::<User, Uuid, SessionPgPool, PgPool>::new(Some(state.pool.clone()))
        .with_config(auth_config),
    )
    .layer(SessionLayer::new(session_store))
    .with_state(state);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  log::info!("listening on http://{}", &addr);
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
