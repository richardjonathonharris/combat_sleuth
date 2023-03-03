use api_server::models::*;
use diesel::prelude::*;
use api_server::establish_connection;

fn main() {
    use api_server::schema::encounters::dsl::*;
    
    let connection = &mut establish_connection();
    let results = encounters
        .limit(5)
        .load::<Encounter>(connection)
        .expect("Error loading encounters");

    println!("Displaying {} encounters", results.len());
    for encounter in results {
        println!("name is {}, created_at is {}, updated_at is {}", 
            encounter.name.unwrap_or("NULL".to_string()),
            encounter.created_at,
            encounter.updated_at
        );
    }
}