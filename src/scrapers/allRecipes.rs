extern crate select;

use crate::error::Error;
use crate::models::{NewRecipe, Recipe};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use indexmap::set::IndexSet;
use rand::Rng;
use regex::Regex;
use reqwest::blocking::get;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::thread;
use std::time::Duration;

pub fn add_recipe(connection: &PgConnection, recipe: &NewRecipe) -> Recipe {
    use crate::schema::recipes_table;

    diesel::insert_into(recipes_table::table)
        .values(recipe)
        .get_result(connection)
        .expect("error saving new recipe")
}

pub fn crawl() {
    use crate::schema::recipes_table::dsl::*;
    let connection: PgConnection = crate::establish_connection();
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
    let mut to_scrape: IndexSet<i32> = get_links(&html_text)
        .difference(&scraped)
        .cloned()
        .collect();

    loop {
        println!("Recipes in scrape queue: {:?}", to_scrape.len());
        let next = to_scrape.pop().unwrap();

        println!("{:?}", next);

        if let Ok((new_recipe, new_urls)) = scrape(next) {
            scraped.insert(next);
            let _added: Recipe = add_recipe(&connection, &new_recipe);
            to_scrape = to_scrape.union(&new_urls).cloned().collect();
            to_scrape = to_scrape.difference(&scraped).cloned().collect();
        }

        let sleep = Duration::from_secs(rng.gen_range(1, 6));
        thread::sleep(sleep);
    }
}

pub fn get_links(html_text: &String) -> IndexSet<i32> {
    // unwrap is fine here because this is a valid regex expression
    let link_regex = Regex::new(r"www.allrecipes.com/recipe/(?P<url_id>\d+)/").unwrap();
    let links = link_regex
        .captures_iter(html_text)
        // unwrap is also fine here because regex group only has digits
        .map(|cap| cap["url_id"].parse().unwrap())
        .collect();
    println!("{:?}", links);
    links
}

// Takes the allrecipes recipe number and returns the recipe
// data as well as all of the linked recipes found on the page.
pub fn scrape(url_id: i32) -> Result<(NewRecipe, IndexSet<i32>), Error> {
    let url = String::from("https://www.allrecipes.com/recipe/") + &url_id.to_string();
    let response = get(url.as_str())?;
    let html_text = response.text()?;
    let links = get_links(&html_text);

    let document = Document::from(html_text.as_str());

    let title;
    if let Some(title_node) = document.find(Class("recipe-summary__h1")).next() {
        title = title_node.text();
    } else {
        return Err(Error::ScrapeError);
    }
    println!("{:?}", title);

    let ingredients: Vec<_> = document
        .find(Class("checkList__line").descendant(Name("label")))
        .filter_map(|ingredient| ingredient.attr("title"))
        .collect();
    let ingredients = ingredients.join("|");
    println!("{:?}", ingredients);

    let time_node = document.find(Class("ready-in-time")).next();
    let time;
    match time_node {
        Some(val) => {
            let time_str = val.text();
            // unwraps are fine here because valid regex expression
            let hours = Regex::new(r"(?P<hours>\d+) h").unwrap();
            let mins = Regex::new(r"(?P<mins>\d+) m").unwrap();
            let hours = match hours.captures(&time_str) {
                // I don't think this can fail
                Some(cap) => cap.name("hours").unwrap().as_str().parse().unwrap(),
                None => 0,
            };
            let mins = match mins.captures(&time_str) {
                // I don't think this can fail
                Some(cap) => cap.name("mins").unwrap().as_str().parse().unwrap(),
                None => 0,
            };
            time = hours as f32 + mins as f32 / 60.0;
        }
        None => time = 0.0,
    }
    println!("{:?}", time);

    let yields_node = document
        .find(Name("meta"))
        .find(|x| x.attr("id") == Some("metaRecipeServings"));
    let yields;
    if let Some(yields_content) = yields_node {
        if let Some(yields_str) = yields_content.attr("content") {
            yields = yields_str.parse()?;
        } else {
            return Err(Error::ScrapeError);
        }
    } else {
        return Err(Error::ScrapeError);
    }
    println!("{:?}", yields);

    let mut agg_rating = document.find(Class("aggregate-rating").descendant(Name("meta")));
    let rating;
    if let Some(rating_node) = agg_rating.next() {
        if let Some(rating_str) = rating_node.attr("content") {
            rating = rating_str.parse()?;
        } else {
            return Err(Error::ScrapeError);
        }
    } else {
        return Err(Error::ScrapeError);
    }
    let reviews;
    if let Some(reviews_node) = agg_rating.next() {
        if let Some(reviews_str) = reviews_node.attr("content") {
            reviews = reviews_str.parse()?;
        } else {
            return Err(Error::ScrapeError);
        }
    } else {
        return Err(Error::ScrapeError);
    }
    println!("{:?}", rating);
    println!("{:?}", reviews);

    let instructions = document
        .find(Class("recipe-directions__list--item"))
        .map(|x| {
            let mut x = x.text();
            let chop = x.find('\n').unwrap_or(x.len());
            x.drain(..chop).collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("|");
    if instructions.is_empty() {
        return Err(Error::ScrapeError);
    }
    println!("{:?}", instructions);

    Ok((
        NewRecipe {
            title,
            time,
            yields,
            ingredients,
            instructions,
            rating,
            reviews,
            url_id,
        },
        links,
    ))
}
