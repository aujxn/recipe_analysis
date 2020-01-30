extern crate diesel;
extern crate recipe_scraper;

use self::diesel::prelude::*;
use self::models::{NewRecipe, Recipe};
use self::recipe_scraper::*;

fn main() {
    use recipe_scraper::schema::recipes_table::dsl::*;
    let connection: PgConnection = establish_connection();

    let test = NewRecipe {
        title: "title".to_string(),
        time: 3.5,
        yields: 8,
        ingredients: "some stuff".to_string(),
        instructions: "some stuff".to_string(),
        rating: 4.7,
        reviews: 155,
        url_id: 12394,
    };

    let _recipe: Recipe = add_recipe(&connection, &test);

    let result = recipes_table
        .limit(5)
        .load::<Recipe>(&connection)
        .expect("failed to query");

    for rec in result {
        println!("{:?}", rec);
    }
}
