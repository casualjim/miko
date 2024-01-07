use std::collections::HashSet;

use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct CurrentUser(pub Option<User>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub id: Uuid,
  pub email: String,
  pub family_name: Option<String>,
  pub given_name: Option<String>,
  pub password: String,
  pub name: String,
  pub picture: Option<String>,
  pub provider: String,
  pub permissions: HashSet<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use axum_session_auth::Authentication;
    use axum_session_auth::HasPermission;
    use async_trait::async_trait;

    use sqlx::PgPool;
    use crate::pgdb::User as SqlUser;

    impl User {
      pub async fn get_by_email(email: &str, pool: &PgPool) -> anyhow::Result<User> {
        let sql_user = SqlUser::get_by_email(email, pool).await?;
        Ok(sql_user)
      }
    }

    #[async_trait]
    impl Authentication<User, Uuid, PgPool> for User {
      async fn load_user(id: Uuid, pool: Option<&PgPool>) -> anyhow::Result<User> {
        let user = SqlUser::get(id, pool.unwrap()).await?;
        Ok(user)
      }
      fn is_authenticated(&self) -> bool {
        true
      }
      fn is_active(&self) -> bool {
        true
      }
      fn is_anonymous(&self) -> bool {
        false
      }
    }

    #[async_trait]
    impl HasPermission<PgPool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
            self.permissions.contains(perm)
        }
    }
  }

}
