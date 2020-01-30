extern crate diesel;

use self::diesel::prelude::*;
use crate::models::Recipe;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::str;

pub struct Ingredient {
    pub name: String,
    pub quantity: Option<String>,
    pub measurement: Option<String>,
}

pub fn pull_recipes() -> Vec<Recipe> {
    use crate::schema::recipes_table::dsl::*;
    let connection: PgConnection = crate::establish_connection();

    recipes_table
        .load::<Recipe>(&connection)
        .expect("failed to query")
        .iter()
        .cloned()
        .collect()
}

pub fn parse_ingredients(recipes: &Vec<Recipe>) -> Vec<Vec<Ingredient>> {
    let phrases: Vec<String> = recipes
        .iter()
        .map(|x| x.ingredients.as_str().split('|'))
        .flatten()
        .map(|st| {
            st.chars()
                .filter(|x| {
                    x.is_ascii_alphanumeric() || x.is_whitespace() || *x == '/' || *x == '.'
                })
                .collect()
        })
        .collect();

    let mut ingredient_file = File::create("ingredients.txt").unwrap();

    let mut to_write = String::new();
    for phrase in phrases {
        to_write += &(phrase
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .join("\n")
            + "\n\n")
    }

    ingredient_file
        .write_all(to_write.as_bytes())
        .expect("file write failed");

    let output = Command::new("/usr/local/bin/crf_test")
        .arg("-m")
        .arg("model.txt")
        .arg("ingredients.txt")
        .output()
        .expect("Conditional Random Field failed");

    let mut all_ingredients: Vec<Vec<Ingredient>> = recipes.iter().map(|_| vec![]).collect();
    let ingredient_counter: Vec<usize> = recipes
        .iter()
        .map(|recipe| recipe.ingredients.as_str().split('|').count())
        .collect();
    let mut counter = 0;
    let mut current_recipe = 0;

    str::from_utf8(&output.stdout)
        .unwrap()
        .split("\n\n")
        .for_each(|phrase| {
            let word_iter = phrase
                .split('\n')
                .map(|word| {
                    let mut iter = word.split_ascii_whitespace();
                    // TODO: this could be handled better
                    (iter.next().unwrap_or("none"), iter.next().unwrap_or("e"))
                })
                .filter(|x| x.1 != "e");

            let mut names: Vec<Option<String>> = vec![None];
            let mut quantity: Option<String> = None;
            let mut measurement: Option<String> = None;

            for (word, key) in word_iter {
                match key {
                    "q" => {
                        if let Some(mut value) = quantity {
                            value.push(' ');
                            value += word;
                            quantity = Some(value);
                        } else {
                            quantity = Some(String::from(word));
                        }
                    }
                    "m" => {
                        if let Some(mut value) = measurement {
                            value.push(' ');
                            value += word;
                            measurement = Some(value);
                        } else {
                            measurement = Some(String::from(word));
                        }
                    }
                    "i" => {
                        let current_ingredient = names.pop();
                        if let Some(Some(mut name)) = current_ingredient {
                            name.push(' ');
                            name += word;
                            names.push(Some(name));
                        } else {
                            let name = Some(String::from(word));
                            names.push(name);
                        }
                    }
                    "a" => names.push(None),
                    _ => panic!("unexpected ingredient label"),
                }
            }

            if counter == ingredient_counter[current_recipe] {
                counter = 0;
                current_recipe += 1;
            }
            counter += 1;

            for ingredient in names {
                if let Some(name) = ingredient {
                    all_ingredients[current_recipe].push(Ingredient {
                        name,
                        quantity: quantity.clone(),
                        measurement: measurement.clone(),
                    });
                }
            }
        });

    all_ingredients
}
