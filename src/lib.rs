#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod co_occurrence;
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

pub fn create_ingredients() {
    use crate::diesel::prelude::*;
    use crate::schema::ingredients;
    use crate::scrapers::nytcooking::Nytc;
    use indexmap::IndexSet;

    #[derive(Insertable)]
    #[table_name = "ingredients"]
    struct Ingredient {
        name: String,
    }

    let all: Vec<Nytc> = crate::process_ny::pull_recipes(vec![], 0, 0, None);
    let recipes: Vec<IndexSet<String>> = crate::process_ny::parse_ingredients(&all);

    let all_ingredients: Vec<Ingredient> = recipes
        .iter()
        .fold(IndexSet::new(), |acc, x| {
            acc.union(x).map(|x| x.clone()).collect()
        })
        .into_iter()
        .map(|x| Ingredient { name: x })
        .collect();

    let connection: PgConnection = crate::establish_connection();

    diesel::insert_into(crate::schema::ingredients::table)
        .values(&all_ingredients)
        .execute(&connection)
        .expect("error inserting new nytc");
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
