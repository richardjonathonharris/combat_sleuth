use api_server::establish_connection;
use api_server::create_encounter;
use std::io::{stdin};


fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();

    println!("Name of encounter?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();

    let encounter = create_encounter(connection, name);
    println!("\nSaved post {} with id {}", name, encounter.id);
}