use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature="ssr")] {
use leptos::LeptosOptions;
use leptos_router::RouteListing;
use leptos_axum::generate_route_list;
use sqlx::PgPool;
use axum::extract::FromRef;
use super::App;
use std::sync::Arc;
use oauth2::{RedirectUrl, RevocationUrl,PkceCodeVerifier,CsrfToken,basic::BasicClient, AuthUrl, ClientId, TokenUrl};
use tokio::sync::RwLock;
use std::time::Duration;


/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
#[derive(FromRef, Clone)]
pub struct AppState {
  pub leptos_options: LeptosOptions,
  pub pool: PgPool,
  pub routes: Vec<RouteListing>,

  pub auth_client: BasicClient,
  secrets: Arc<RwLock<ttl_cache::TtlCache<String, PkceCodeVerifier>>>,
  // openai_client: Arc<rs_openai::OpenAI>,
}

impl AppState {
  pub async fn new<S: Into<String>>(database_url: S, client_id: S, leptos_options: LeptosOptions) -> crate::Result<Self> {
    let pool = PgPool::connect(&database_url.into()).await?;
    let routes = generate_route_list(App);
    Ok(Self {
      leptos_options,
      pool,
      routes,
      secrets: Arc::new(RwLock::new(ttl_cache::TtlCache::new(100_000))),
      auth_client: BasicClient::new(
        ClientId::new(client_id.into()),
        None,
        AuthUrl::new("https://wagyu-tunhvc.zitadel.cloud/oauth/v2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://wagyu-tunhvc.zitadel.cloud/oauth/v2/token".to_string()).unwrap()),
      ),
    })
  }

  pub fn with_redirect_url<S: Into<String>>(mut self, redirect_url: S) -> Self {
    self.auth_client = self
      .auth_client
      .set_redirect_uri(RedirectUrl::new(redirect_url.into()).expect("invalid redirect_url"));
    self
  }

  pub fn with_revocation_url<S: Into<String>>(mut self, revocation_url: S) -> Self {
    self.auth_client = self.auth_client.set_revocation_uri(
      RevocationUrl::new(revocation_url.into()).expect("invalid revocation_url"),
    );
    self
  }

  pub fn auth_client(&self) -> BasicClient {
    self.auth_client.clone()
  }

  // pub fn openai_client(&self) -> Arc<rs_openai::OpenAI> {
  //   self.openai_client.clone()
  // }

  pub(crate) async fn remember_verifier(&self, token: &CsrfToken, verifier: PkceCodeVerifier) {
    self.secrets.write().await.insert(
      token.secret().to_string(),
      verifier,
      Duration::from_secs(15 * 60),
    );
  }

  pub(crate) async fn retrieve_verifier(&self, token: &str) -> Option<PkceCodeVerifier> {
    self.secrets.write().await.remove(token)
  }
}

  }}