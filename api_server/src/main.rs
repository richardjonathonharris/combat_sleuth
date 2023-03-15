mod entities;
mod setup;

use entities::{prelude::*, *};
use rocket::{
    fs::{relative, FileServer},
    serde::{Serialize, json::Json},
    *,
};
use rocket_dyn_templates::Template;
use sea_orm::*;
use serde_json::json;
use setup::set_up_db;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Index {}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct MonsterJson {
    id: i32,
    name: String,
    hp: i32,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct MonstersJson {
    monsters: Vec<MonsterJson>,
    count_monsters: i32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorJson {
    error: String,
}

#[get("/")]
fn index() -> Json<Index> {
    Json(Index {})
}

#[get("/monsters")]
async fn monsters(db: &State<DatabaseConnection>) -> Result<Json<MonstersJson>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let monsters = Monster::find()
        .all(db)
        .await?
        .into_iter()
        .map(|m| MonsterJson{ id: m.id, name: m.name, hp: m.hp})
        .collect::<Vec<_>>();

    let monsters_count = monsters.len();

    Ok(
        Json(MonstersJson { 
            monsters: monsters, 
            count_monsters: monsters_count as i32,
        })
    )
}

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .mount(
            "/", 
            routes![index, monsters],
        )
        .register("/", catchers![not_found])
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Json<ErrorJson> {
    Json(ErrorJson { error: "The requested url was not found.".to_string() })
}

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct ErrorResponder {
    message: String,
}

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        ErrorResponder {
            message: err.to_string(),
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        str.to_owned().into()
    }
}