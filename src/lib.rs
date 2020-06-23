#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod embed;
pub mod error;
pub mod expanded;
pub mod hierarchy;
pub mod louvain;
pub mod models;
pub mod process;
pub mod process_ny;
pub mod recipe;
pub mod schema;
pub mod scrapers;
pub mod table;
pub mod training;

use self::models::{NytcQueryable, Recipe};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var not set");

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn dump_ny() {
    use self::schema::nyt::dsl::*;
    let connection: PgConnection = establish_connection();

    let all: Vec<NytcQueryable> = nyt
        .load::<NytcQueryable>(&connection)
        .expect("failed")
        .into_iter()
        .collect();

    let json = serde_json::to_string(&all).unwrap();

    let mut file = File::create("nytcdump.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn dump_allrecipes() {
    use self::schema::recipes_table::dsl::*;
    let connection: PgConnection = establish_connection();

    let all: Vec<Recipe> = recipes_table
        .load::<Recipe>(&connection)
        .expect("failed")
        .into_iter()
        .collect();

    let json = serde_json::to_string(&all).unwrap();

    let mut file = File::create("allrecipesdotcomdump.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
