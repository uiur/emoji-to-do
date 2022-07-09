use serde::Serialize;
use sqlx::SqlitePool;



#[derive(Debug, Serialize)]
pub struct User {
  pub id: i64,
  pub slack_team_id: String,
  pub slack_user_id: String,
  pub slack_token: String,
  pub created_at: String,
}

impl User {
  pub async fn find(connection: &SqlitePool, id: i64) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "
      select * from users where id = ? limit 1
    ", id
    ).fetch_optional(connection).await
  }

  pub async fn find_by_slack_user_id(connection: &SqlitePool, slack_user_id: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "
      select * from users where slack_user_id = ? limit 1
    ", slack_user_id
    ).fetch_optional(connection).await
  }
  pub async fn create(connection: &SqlitePool, slack_team_id: &str, slack_user_id: &str, slack_token: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!("
      insert into users (slack_team_id, slack_user_id, slack_token)
        values (?, ?, ?)
    ", slack_team_id, slack_user_id, slack_token).execute(connection).await?;

    Ok(result.last_insert_rowid())
  }
}
