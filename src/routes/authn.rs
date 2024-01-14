use leptos::*;

#[cfg(feature = "ssr")] use crate::app::{app_state, auth};
use crate::models::User;

#[server(GetUser, "/bff")]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
  let auth = auth()?;
  Ok(auth.current_user)
}

#[server(OauthLogin, "/bff")]
pub async fn oauth_login() -> Result<(), ServerFnError> {
  let app_state = app_state()?;
  // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
  // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
  let (pkce_code_challenge, pkce_code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

  // Generate the authorization URL to which we'll redirect the user.
  let (authorize_url, csrf_state) = app_state
    .auth_client()
    .authorize_url(oauth2::CsrfToken::new_random)
    .add_scope(oauth2::Scope::new("openid".to_string()))
    .add_scope(oauth2::Scope::new("email".to_string()))
    .add_scope(oauth2::Scope::new("profile".to_string()))
    .set_pkce_challenge(pkce_code_challenge)
    .url();

  tracing::info!("csrf state: {}", csrf_state.secret());
  app_state
    .remember_verifier(&csrf_state, pkce_code_verifier)
    .await;
  leptos_axum::redirect(authorize_url.as_str());
  Ok(())
}

#[server(Logout, "/bff")]
pub async fn logout() -> Result<(), ServerFnError> {
  let auth = auth()?;

  auth.logout_user();
  leptos_axum::redirect("/");

  Ok(())
}
