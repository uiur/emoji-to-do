use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, SqlitePool};

use super::{reaction::Reaction, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionAssignee {
    pub id: i64,
    pub name: String,
    pub reaction_id: i64,
}

impl ReactionAssignee {
    pub async fn find(connection: &SqlitePool, id: i64) -> Result<Option<ReactionAssignee>, Error> {
        let result = sqlx::query_as!(
            ReactionAssignee,
            "
    select id, name, reaction_id
    from reaction_assignees
    where id = ?
    limit 1
    ",
            id
        )
        .fetch_optional(connection)
        .await?;

        Ok(result)
    }

    pub async fn search_by_reaction_id(
        connection: &SqlitePool,
        reaction_id: i64,
    ) -> Result<Vec<ReactionAssignee>, Error> {
        let result = sqlx::query_as!(
            ReactionAssignee,
            "
select id, name, reaction_id
from reaction_assignees
where reaction_id = ?
order by id
",
            reaction_id
        )
        .fetch_all(connection)
        .await?;

        Ok(result)
    }

    pub async fn create(
        connection: &SqlitePool,
        reaction_id: i64,
        name: &str,
    ) -> Result<i64, Error> {
        let result = sqlx::query!(
            "
      insert into reaction_assignees (reaction_id, name)
      values (?, ?)
      ",
            reaction_id,
            name
        )
        .execute(connection)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn destroy(connection: &SqlitePool, id: i64) -> Result<(), Error> {
        let result = sqlx::query!(
            "
    delete from reaction_assignees where id = ?",
            id
        )
        .execute(connection)
        .await?;
        Ok(())
    }
}
