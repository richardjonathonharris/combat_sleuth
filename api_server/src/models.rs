use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::encounters;

#[derive(Queryable)]
pub struct Encounter {
    pub id: i32,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = encounters)]
pub struct NewEncounter<'a> {
    pub name: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}