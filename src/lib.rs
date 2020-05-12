#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod embed;
pub mod error;
pub mod expanded;
pub mod hierarchy;
pub mod louvain;
pub mod models;
pub mod nytc;
pub mod process;
pub mod process_ny;
pub mod schema;
pub mod scraper;
pub mod table;
pub mod training;

use self::models::{NewRecipe, Recipe};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var not set");

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}
