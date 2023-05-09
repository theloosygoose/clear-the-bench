use sqlx::SqlitePool;

pub async fn create_teams_table(db: &SqlitePool) {
    match sqlx::query(
        "CREATE TABLE IF NOT EXISTS teams
        (
            team_id     VARCHAR(250) PRIMARY KEY NOT NULL,
            name        VARCHAR(100)             NOT NULL,
            city_name   VARCHAR(100)             NOT NULL,
            longitude   REAL                     NOT NULL,
            latitude    REAL                     NOT NULL,
            owner       VARCHAR(100),
            coach       VARCHAR(100),
            wins        INTEGER                  NOT NULL DEFAULT 0,
            losses      INTEGER                  NOT NULL DEFAULT 0,
            team_salary INTEGER                  NOT NULL DEFAULT 0
        )
        ",
    )
    .execute(db)
    .await
    {
        Ok(val) => val,
        Err(error) => panic!("Could not create teams table:: {}", error),
    };
}
