use sqlx::SqlitePool;

use crate::database_handlers::getperson::GetPerson;
use crate::people::Person;

pub async fn insert_person(person: Person, db: &SqlitePool) {
    let result = sqlx::query(
        "INSERT INTO people 
        (person_id, name, country, age, active, job, team, 
        
        work_ethic, intelligence, creativity, 
        adaptability, loyalty, dog_factor,   
        
        strength, fluidity, burst, speed, 
        height, wingspan, off_awareness, def_awareness,
        shot_form, touch, pass_accuracy, ball_handling, 
        sliding, hands)
        VALUES (?,?,?,?,?,?,?,
                ?,?,?,
                ?,?,?,
                ?,?,?,?,
                ?,?,?,?,
                ?,?,?,?,
                ?,?)",
    )
    .bind(person.person_id)
    .bind(person.name)
    .bind(person.country.to_string())
    .bind(person.age)
    .bind(person.active)
    .bind(person.job.to_string())
    .bind(person.team)
    .bind(person.personality.work_ethic)
    .bind(person.personality.intelligence)
    .bind(person.personality.creativity)
    .bind(person.personality.adaptability)
    .bind(person.personality.loyalty)
    .bind(person.personality.dog_factor)
    .bind(person.intangibles.strength)
    .bind(person.intangibles.fluidity)
    .bind(person.intangibles.burst)
    .bind(person.intangibles.speed)
    .bind(person.intangibles.height)
    .bind(person.intangibles.wingspan)
    .bind(person.intangibles.off_awareness)
    .bind(person.intangibles.def_awareness)
    .bind(person.intangibles.shot_form)
    .bind(person.intangibles.touch)
    .bind(person.intangibles.pass_accuracy)
    .bind(person.intangibles.ball_handling)
    .bind(person.intangibles.sliding)
    .bind(person.intangibles.hands)
    .execute(db)
    .await
    .unwrap();
}

pub async fn get_people(db: &SqlitePool) -> Vec<Person> {
    let mut people = vec![];

    let people_results = sqlx::query_as::<_, GetPerson>("SELECT * FROM people")
        .fetch_all(db)
        .await
        .unwrap();

    for person_db in people_results {
        let person = person_db.translate_to_person();
        people.push(person);
    }

    return people;
}
