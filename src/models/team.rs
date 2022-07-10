use sqlx::SqlitePool;

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
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

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
}
