mod entities;
mod setup;

use entities::{prelude::*, *};
use rocket::{
    fs::{relative, FileServer},
    *,
};
use rocket_dyn_templates::Template;
use sea_orm::*;
use serde_json::json;
use setup::set_up_db;

#[get("/")]
fn index() -> Template {
    Template::render("index", json!({}))
}

#[get("/monsters")]
async fn monsters(db: &State<DatabaseConnection>) -> Result<Template, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let monsters = Monster::find()
        .all(db)
        .await?
        .into_iter()
        .map(|m| json!({ "name": m.name, "id": m.id }))
        .collect::<Vec<_>>();

    Ok(Template::render(
        "monsters",
        json!({ "monsters": monsters, "num_monsters": monsters.len() }),
    ))
}

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .mount("/", FileServer::from(relative!("/static")))
        .mount(
            "/", 
            routes![index, monsters],
        )
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        json! ({
            "uri": req.uri()
        })
    )
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