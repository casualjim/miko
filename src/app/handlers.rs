use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature="ssr")] {

use axum::{
    response::{Response, IntoResponse, Redirect},
    extract::{Path, State,Query},
    http::{Request},
    body::Body as AxumBody,
};

use leptos::*;
use leptos_axum::handle_server_fns_with_context;
use axum_session_auth::SessionPgPool;
use sqlx::PgPool;
use crate::app::App;
use crate::app::state::AppState;
use uuid::Uuid;
use serde::Deserialize;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, Scope, TokenResponse};
use crate::pgdb::UserInfo;
use crate::models::User;

pub type AuthSession = axum_session_auth::AuthSession<User, Uuid, SessionPgPool, PgPool>;
}}

cfg_if! {
  if #[cfg(feature = "ssr")] {

    pub async fn server_fn_handler(
      State(app_state): State<AppState>,
      auth_session: AuthSession,
      path: Path<String>,
      request: Request<AxumBody>) -> impl IntoResponse {

        tracing::info!("{} {:?}", request.method(), path);

        handle_server_fns_with_context(move || {
            provide_context(auth_session.clone());
            provide_context(app_state.pool.clone());
            provide_context(app_state.clone());
        }, request).await
    }

    pub async fn leptos_routes_handler(auth_session: AuthSession, State(app_state): State<AppState>, req: Request<AxumBody>) -> Response{
        let handler = leptos_axum::render_route_with_context(
          app_state.leptos_options.clone(),
          app_state.routes.clone(),
          move || {
              provide_context(auth_session.clone());
              provide_context(app_state.pool.clone());
              provide_context(app_state.clone());
          },
          App
        );
        handler(req).await.into_response()
    }

    pub async fn start_login(State(app_state): State<AppState>) -> crate::Result<impl IntoResponse> {
      // Zitadel supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
      // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
      let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

      let redirect_uri = oauth2::RedirectUrl::new("http://localhost:3000/oauth/finish".to_string()).expect("invalid redirect_url");
      let rurl = std::borrow::Cow::Owned(redirect_uri);

      // Generate the authorization URL to which we'll redirect the user.
      let (authorize_url, csrf_state) = app_state
        .auth_client()
        .authorize_url(CsrfToken::new_random)
        .set_redirect_uri(rurl)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

      tracing::info!("csrf state: {}", csrf_state.secret());
      app_state.remember_verifier(&csrf_state, pkce_code_verifier).await;
      Ok(Redirect::to(authorize_url.as_ref()))
    }

    #[derive(Debug, Deserialize)]
    pub struct AuthRequest {
      code: String,
      state: String,
    }

    pub async fn get_access_token(
      Query(query): Query<AuthRequest>,
      session: AuthSession,
      State(app_state): State<AppState>,
    ) -> crate::Result<impl IntoResponse> {
      let pkce_verifier = app_state.retrieve_verifier(&query.state).await.unwrap();
      let token = app_state
        .auth_client()
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .unwrap();

      let secret = token.access_token().secret();
      let client = ::reqwest::Client::new();
      let user_data = client
        .get("https://wagyu-tunhvc.zitadel.cloud/oidc/v1/userinfo")
        .bearer_auth(secret)
        .send()
        .await
        .unwrap()
        .json::<UserInfo>()
        .await
        .unwrap();

      tracing::info!("the user data: {user_data:?}");

      let user = UserInfo::save(&app_state.pool, user_data.clone()).await?;
      session.login_user(user.id);
      // session.insert(USER_SESSION_FIELD, user_data)?;

      Ok(Redirect::to("/"))
    }

}}
