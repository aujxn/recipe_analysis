extern crate diesel;

use self::diesel::prelude::*;
use crate::models::NytcQueryable;
use crate::scrapers::nytcooking::Nytc;
use indexmap::IndexSet;
use itertools::Itertools;
use serde_json::de::from_str;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
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

pub fn parse_ingredients(recipes: &Vec<Nytc>) -> Vec<IndexSet<String>> {
    let phrases = get_phrases(recipes);
    let mut temp_file = File::create("../../ingredient-phrase-tagger/temp/phrases").unwrap();
    temp_file.write_all(phrases.as_bytes()).unwrap();

    let _ = Command::new("sh").arg("parse.sh").output().unwrap();
    //print!("{}", &str::from_utf8(&result.stdout).unwrap());

    let mut labeled = File::open("./temp/labeled").unwrap();
    let mut labeled_contents = String::new();
    labeled.read_to_string(&mut labeled_contents).unwrap();

    let ingredient_count: Vec<usize> = recipes
        .iter()
        .map(|recipe| {
            recipe
                .ingredients
                .ingredients
                .iter()
                .fold(
                    0,
                    |acc, (_, phrase)| {
                        if phrase.trim() != "" {
                            acc + 1
                        } else {
                            acc
                        }
                    },
                )
        })
        .collect();

    let ingredients: Vec<Option<String>> = from_str::<Vec<Value>>(&labeled_contents)
        .unwrap()
        .iter()
        .map(|obj| {
            if let Value::Object(values) = obj {
                if let Some(name) = values.get("name") {
                    Some(name.as_str().unwrap().trim_matches('"').to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let mut split = vec![];
    let mut i = 0;

    for count in ingredient_count {
        let mut next = IndexSet::new();
        for _ in 0..count {
            if let Some(ing) = &ingredients[i] {
                next.insert(ing.clone());
            }
            i += 1;
        }
        split.push(next);
    }

    split
}

fn get_phrases(recipes: &Vec<Nytc>) -> String {
    let phrases: Vec<Vec<String>> = recipes
        .iter()
        .map(|recipe| {
            recipe
                .ingredients
                .ingredients
                .iter()
                .filter_map(|(qty, phrase)| {
                    let new_qty;
                    let new_phrase;
                    if phrase.trim() == "" {
                        None
                    } else {
                        if qty.trim() != "" {
                            new_qty = String::from(qty.trim()) + " ";
                        } else {
                            new_qty = String::from("");
                        }
                        new_phrase = phrase.replace("\n", " ");
                        let mut final_phrase =
                            unicode_ascii(&(new_qty + &new_phrase)).trim().to_string();
                        final_phrase.make_ascii_lowercase();
                        Some(final_phrase)
                    }
                })
                .collect()
        })
        .collect();

    phrases
        .iter()
        .map(|recipe| recipe.iter().join("\n"))
        .join("\n")
}

fn unicode_ascii(phrase: &str) -> String {
    let unicode_frac = vec![
        "⅛", "⅜", "⅝", "⅞", "⅙", "⅚", "⅕", "⅖", "⅗", "⅘", "¼", "¾", "⅓", "⅔", "½",
    ];
    let ascii_frac = vec![
        " 1/8", " 3/8", " 5/8", " 7/8", " 1/6", " 5/6", " 1/5", " 2/5", " 3/5", " 4/5", " 1/4",
        " 3/4", " 1/3", " 2/3", " 1/2",
    ];

    let mut new: String = phrase.to_string();

    // Replace all unicode fractions with ascii
    for (unicode, ascii) in unicode_frac.iter().zip(ascii_frac) {
        if new.contains(unicode) {
            new = new.replace(unicode, ascii);
        }
    }

    new.split_ascii_whitespace().join(" ")
}
