use sqlx::SqlitePool;

use super::{reaction::Reaction, Error};

#[derive(Debug)]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub slack_team_id: String,
}

impl Team {
    pub async fn find(
        connection: &SqlitePool,
        slack_team_id: &str,
    ) -> Result<Option<Team>, sqlx::Error> {
        sqlx::query_as!(
            Team,
            "
      select id, name, slack_team_id from teams where slack_team_id = ? limit 1
    ",
            slack_team_id
        )
        .fetch_optional(connection)
        .await
    }

    pub async fn find_by_id(connection: &SqlitePool, id: i64) -> Result<Option<Team>, Error> {
        sqlx::query_as!(
            Team,
            "
      select id, name, slack_team_id from teams where id = ? limit 1
    ",
            id
        )
        .fetch_optional(connection)
        .await
        .map_err(|e| e.into())
    }

    pub async fn create(
        connection: &SqlitePool,
        name: &str,
        slack_team_id: &str,
    ) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            "
            insert into teams (name, slack_team_id)
            values (?, ?)
        ",
            name,
            slack_team_id
        )
        .execute(connection)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn reactions(&self, connection: &SqlitePool) -> Result<Vec<Reaction>, sqlx::Error> {
        let result = sqlx::query_as!(
            Reaction,
            "
            select id, name, team_id, repo
            from reactions
            where team_id = ?
            order by id
        ",
            self.id
        )
        .fetch_all(connection)
        .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

    use crate::slack;

    use super::Reaction;
    use super::Team;

    async fn setup_db() -> SqlitePool {
        let connection = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("failed to open database");

        sqlx::migrate!("./migrations")
            .run(&connection)
            .await
            .expect("failed to migrate database");

        connection
    }

    #[actix_rt::test]
    async fn team_find() {
        let connection = setup_db().await;
        let result = Team::find(&connection, "not_found").await;
        match result {
            Ok(t) => {
                assert_matches!(t, None)
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[actix_rt::test]
    async fn team_reactions() -> Result<(), Box<dyn std::error::Error>> {
        let connection = setup_db().await;
        let slack_team_id = "team_id";
        let team_id = Team::create(&connection, "team_name", slack_team_id).await?;
        Reaction::create(&connection, team_id, "eyes", "uiur/sandbox").await?;

        let team = Team::find(&connection, slack_team_id).await?.unwrap();
        let reactions = team.reactions(&connection).await?;
        assert_eq!(reactions.len(), 1);
        Ok(())
    }
}
