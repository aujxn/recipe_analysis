extern crate diesel;

use self::diesel::prelude::*;
use crate::models::NytcQueryable;
use crate::scrapers::nytcooking::Nytc;
use std::str;

pub fn pull_recipes(
    q_tags: Vec<String>,
    q_votes: i32,
    q_rating: i32,
    q_author: Option<String>,
) -> Vec<Nytc> {
    use crate::schema::nyt::dsl::*;
    let connection: PgConnection = crate::establish_connection();

    let mut query = nyt.into_boxed();

    if let Some(q_author) = q_author {
        query = query.filter(author.eq(q_author));
    }

    if q_votes > 0 {
        query = query.filter(ratings.ge(q_votes));
    }

    if q_rating > 0 {
        query = query.filter(rating.ge(q_rating));
    }

    if q_tags.len() > 0 {
        query = query.filter(tags.contains(q_tags));
    }

    let data: Vec<Nytc> = query
        .load::<NytcQueryable>(&connection)
        .expect("failed to query")
        .into_iter()
        .map(move |x| Nytc::new(x))
        .collect();

    data
}

pub fn filter_title(recipes: Vec<Nytc>, title: &str) -> Vec<Nytc> {
    let title = String::from(title);
    recipes
        .into_iter()
        .filter(|recipe| {
            let title = title.to_ascii_lowercase();
            let recipe = recipe.title.to_ascii_lowercase();
            recipe.contains(&title)
        })
        .collect()
}
