use sqlx::SqlitePool;

pub async fn create_people_table(db: &SqlitePool) {
    let create_people_table = match sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS people
        (
          person_id   VARCHAR(250) PRIMARY KEY    NOT NULL,
          name        VARCHAR(100)                NOT NULL,
          country     VARCHAR(100)                NOT NULL,
          age         INTEGER                     NOT NULL,
          active      INTEGER                     NOT NULL DEFAULT 1,
          job         VARCHAR(50)                 NOT NULL,
          team        VARCHAR(100),
          work_ethic    INTEGER,
          intelligence  INTEGER, 
          creativity    INTEGER, 
          adaptability  INTEGER, 
          loyalty       INTEGER, 
          dog_factor    INTEGER, 
          strength      INTEGER,
          fluidity      INTEGER,
          burst         INTEGER,
          speed         INTEGER,
          height        INTEGER,
          wingspan      INTEGER,
          off_awareness INTEGER,
          def_awareness INTEGER,
          shot_form     INTEGER,
          touch         INTEGER,
          pass_accuracy INTEGER,
          ball_handling INTEGER,
          sliding       INTEGER,
          hands         INTEGER
        )",
    )
    .execute(db)
    .await
    {
        Ok(val) => val,
        Err(error) => panic!("Could not create people table:: {}", error),
    };
}
