
use sqlx::PgPool;
use sqlx::types::chrono;
use sqlx::types::chrono::Utc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::models::User as AppUser;
use crate::{Result,Error};

#[derive(
  Debug,
  Deserialize,
  Serialize,
  Clone,
  sqlx::FromRow,
)]
pub struct User {
  pub id: Uuid,
  pub email: String,
  pub family_name: Option<String>,
  pub given_name: Option<String>,
  pub name: String,
  pub picture: Option<String>,
  pub provider: String,
  pub created_at: chrono::DateTime<Utc>,
  pub updated_at: chrono::DateTime<Utc>,
}

impl User {

  pub async fn get(id: Uuid, pool: &PgPool) -> Result<AppUser> {
    let sql_user = sqlx::query_as!(
      User,
      r#"
        SELECT id, email, family_name, given_name, name, picture, provider, created_at, updated_at
        FROM users
        WHERE id = $1
      "#,
      id
    ).fetch_one(pool);

    let sql_user_perms = sqlx::query_as!(
      SqlPermissionTokens,
      r#"
        SELECT token
        FROM user_permissions
        WHERE user_id = $1
      "#,
      id
    ).fetch_all(pool);

    let (sql_user, sql_user_perms) = futures::try_join!(sql_user, sql_user_perms).map_err(Error::Pgx)?;

    Ok(sql_user.into_user(sql_user_perms))
  }

  pub async fn get_by_email(email: &str, pool: &PgPool) -> Result<AppUser> {
    let sql_user = sqlx::query_as!(
      User,
      r#"
        SELECT id, email, family_name, given_name, name, picture, provider, created_at, updated_at
        FROM users
        WHERE email = $1
      "#,
      email
    ).fetch_one(pool).await.map_err(Error::Pgx)?;

    let sql_user_perms = sqlx::query_as!(
      SqlPermissionTokens,
      r#"
        SELECT token
        FROM user_permissions
        WHERE user_id = $1
      "#,
      sql_user.id
    ).fetch_all(pool).await.map_err(Error::Pgx)?;

    Ok(sql_user.into_user(sql_user_perms))
  }


  pub fn into_user(self, sql_user_perms: Vec<SqlPermissionTokens>) -> AppUser {
    AppUser {
      id: self.id,
      email: self.email,
      family_name: self.family_name,
      given_name: self.given_name,
      password: String::new(),
      name: self.name,
      picture: self.picture,
      provider: self.provider,
      created_at: self.created_at,
      updated_at: self.updated_at,
      permissions: sql_user_perms.into_iter().map(|p| p.token).collect(),
    }
  }
}

impl Default for User {
  fn default() -> Self {
    Self {
      id: Uuid::new_v4(),
      email: "".into(),
      family_name: None,
      given_name: None,
      name: "".into(),
      picture: None,
      provider: "".into(),
      created_at: Utc::now(),
      updated_at: Utc::now(),
    }
  }
}


#[derive(sqlx::FromRow, Clone)]
pub struct SqlPermissionTokens {
    pub token: String,
}


#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct UserInfo {
  pub email: String,
  pub family_name: Option<String>,
  pub given_name: Option<String>,
  pub name: String,
  pub picture: Option<String>,
}

impl UserInfo {
  pub async fn save(
    conn: &PgPool,
    UserInfo {
      email,
      name,
      picture,
      family_name,
      given_name,
    }: UserInfo,
  ) -> Result<AppUser> {
    let res = sqlx::query_file_as!(
      UserInfo,
      "queries/upsert_user_account.sql",
      email,
      name,
      picture,
      family_name,
      given_name,
    )
    .fetch_one(conn)
    .await?;

    User::get_by_email(&email, conn).await
  }
}
