pub mod models;
pub mod schema;

use chrono::offset::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewEncounter, Encounter};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_encounter(conn: &mut PgConnection, name: &str) -> Encounter {
    use crate::schema::encounters;

    let new_encounter = NewEncounter { name, created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc()};

    diesel::insert_into(encounters::table)
        .values(&new_encounter)
        .get_result(conn)
        .expect("Error saving new encounter")
}