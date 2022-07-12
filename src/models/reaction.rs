use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Debug, Serialize)]
pub struct Reaction {
    pub id: i64,
    pub name: String,
    pub team_id: i64,
    pub repo: String,
}

impl Reaction {
    pub async fn find(connection: &SqlitePool, id: i64) -> Result<Option<Reaction>, sqlx::Error> {
        sqlx::query_as!(
            Reaction,
            "
        select id, name, team_id, repo from reactions where id = ? limit 1
        ",
            id
        )
        .fetch_optional(connection)
        .await
    }

    pub async fn create(
        connection: &SqlitePool,
        team_id: i64,
        name: &str,
        repo: &str,
    ) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            "
        insert into reactions (team_id, name, repo)
        values (?, ?, ?);
        ",
            team_id,
            name,
            repo
        )
        .execute(connection)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn find_by_team_id_and_name(
        connection: &SqlitePool,
        team_id: i64,
        name: &str,
    ) -> Result<Option<Reaction>, sqlx::Error> {
        sqlx::query_as!(
            Reaction,
            "
      select id, name, team_id, repo
      from reactions
      where team_id = ? and name = ?
      order by id
      limit 1
    ",
            team_id,
            name
        )
        .fetch_optional(connection)
        .await
    }
}
