extern crate diesel;

use self::diesel::prelude::*;
use crate::models::NytcQueryable;
use crate::nytc::Nytc;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::str;

pub fn pull_recipes() -> Vec<Nytc> {
    use crate::schema::nyt::dsl::*;
    let connection: PgConnection = crate::establish_connection();

    let data: Vec<Nytc> = nyt
        .load::<NytcQueryable>(&connection)
        .expect("failed to query")
        .into_iter()
        .map(move |x| Nytc::new(x))
        .collect();

    println!("recipes pulled: {}", data.len());
    data
}
