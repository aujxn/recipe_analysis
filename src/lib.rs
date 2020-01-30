#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod error;
pub mod models;
pub mod process;
pub mod schema;
pub mod scraper;
pub mod table;

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

pub fn add_recipe(connection: &PgConnection, recipe: &NewRecipe) -> Recipe {
    use schema::recipes_table;

    diesel::insert_into(recipes_table::table)
        .values(recipe)
        .get_result(connection)
        .expect("error saving new recipe")
}
