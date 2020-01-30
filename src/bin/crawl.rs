extern crate diesel;
extern crate rand;
extern crate recipe_scraper;

use self::diesel::prelude::*;
use self::models::{NewRecipe, Recipe};
use self::recipe_scraper::*;
use indexmap::set::IndexSet;
use rand::Rng;
use reqwest::blocking::get;
use std::thread;
use std::time::Duration;

fn main() {
    use recipe_scraper::schema::recipes_table::dsl::*;
    let connection: PgConnection = establish_connection();
    let mut rng = rand::thread_rng();

    let mut scraped: IndexSet<i32> = recipes_table
        .load::<Recipe>(&connection)
        .expect("failed to query")
        .iter()
        .map(|recipe| recipe.url_id)
        .collect();
    println!("Recipes in DB: {:?}", scraped.len());

    let url = String::from("https://www.allrecipes.com/recipes/17562/dinner/");
    let response = get(url.as_str()).unwrap();
    let html_text = response.text().unwrap();
    let mut to_scrape: IndexSet<i32> = scraper::get_links(&html_text)
        .difference(&scraped)
        .cloned()
        .collect();

    loop {
        println!("Recipes in scrape queue: {:?}", to_scrape.len());
        let next = to_scrape.pop().unwrap();

        println!("{:?}", next);

        if let Ok((new_recipe, new_urls)) = recipe_scraper::scraper::scrape(next) {
            scraped.insert(next);
            let _added: Recipe = add_recipe(&connection, &new_recipe);
            to_scrape = to_scrape.union(&new_urls).cloned().collect();
            to_scrape = to_scrape.difference(&scraped).cloned().collect();
        }

        let sleep = Duration::from_secs(rng.gen_range(1, 6));
        thread::sleep(sleep);
    }
}
