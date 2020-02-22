extern crate recipe_analysis;

//use recipe_analysis::models::Recipe;
use recipe_analysis::models::NytcQueryable;
use recipe_analysis::nytc::Nytc;
use recipe_analysis::training::unicode_ascii;
use serde::{Deserialize, Serialize};
use serde_json::de::from_str;
use serde_json::Value;
//use recipe_analysis::process::*;
use indexmap::IndexSet;
use recipe_analysis::process_ny;
use recipe_analysis::table::*;
use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "tool for recipe analysis")]
enum Opt {
    Scrape {
        /// Argument must be nytc or allrecipes
        #[structopt(short, long)]
        website: String,
    },
    AnalyzeNYT {
        /// Filter by tags
        #[structopt(short, long)]
        tags: Vec<String>,

        /// Filter by ingredients
        #[structopt(short, long)]
        ingredients: Vec<String>,

        /// Filter by minimum vote count
        #[structopt(short, long, default_value = "0")]
        votes: i32,

        /// Filter by minimum rating (0 to 5)
        #[structopt(short, long, default_value = "0")]
        rating: i32,

        /// Filter by author
        #[structopt(short, long)]
        author: Option<String>,

        /// Title must contain the substring
        #[structopt(short, long)]
        title: Option<String>,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Scrape { website } => (),
        Opt::AnalyzeNYT {
            tags,
            ingredients,
            votes,
            rating,
            author,
            title,
        } => {
            let mut recipes = process_ny::pull_recipes(tags, votes, rating, author);

            if let Some(title) = title {
                recipes = process_ny::filter_title(recipes, &title)
            }

            let parsed_ingredients = parse_ingredients(&recipes);
            /*
            if ingredients.len() > 0 {
                let ingredients: IndexSet<String> = ingredients.into_iter().collect();
                filtered = filtered
                    .into_iter()
                    .filter(|recipe| ingredients.is_subset(&recipe.ingredients.ingredients))
                    .collect();
            }
            */
        }
    }
    /*
        recipes.retain(|x| {
            x.tags
                .iter()
                .find(|tag| tag.contains("Vegetarian"))
                .is_some()
                && x.tags
                    .iter()
                    .find(|tag| tag.contains("Breakfast"))
                    .is_some()
                && x.rating as u32 == 5
                && x.ratings > 1000
        });
    */

    /*
    for recipe in ingredients {
        for line in recipe {
            println!("{}", line);
        }
    }
    */

    let mut file = File::open("labeled").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let ingredient_count: Vec<i32> = recipes
        .iter()
        .map(|recipe| recipe.ingredients.ingredients.len())
        .collect();

    let ingredients: Vec<Option<String>> = from_str::<Vec<Value>>(&contents)
        .unwrap()
        .iter()
        .map(|obj| {
            if let Value::Object(values) = obj {
                if let Some(name) = values.get("name") {
                    Some(name.to_string())
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
    for (j, recipe) in phrases.iter().enumerate() {
        split.push(vec![]);
        for _ in recipe {
            if let Some(ing) = &ingredients[i] {
                split[j].push(ing.clone());
            }
            i += 1;
        }
    }

    let filtered: Vec<Vec<String>> = recipes
        .into_iter()
        .zip(split.into_iter())
        .filter_map(|(recipe, ingredients)| {
            if recipe.tags.iter().find(|&tag| tag == "Cocktails").is_some() {
                Some(ingredients)
            } else {
                None
            }
        })
        .collect();

    let (ingredients_map, ingredients_vec, ingredients_count, ingredient_cooccurrence) =
        ingredient_map(&filtered);

    for ingredient in ingredients_vec {
        println!("{}", ingredient);
    }
    /*
    for (x, y, count) in ingredient_cooccurrence.points {
        println!("{} {} {}", x, y, count);
    }
    */
    /*
    for recipe in recipes.iter().take(5) {
        for phrase in recipe.ingredients.ingredients.iter() {
            if phrase.0 != "" {
                print!("{} ", phrase.0);
            }
            println!("{}", phrase.1);
        }
        println!();
    }
    */

    //println!("count: {}\n", recipes.len());

    /* CRF format
    for recipe in recipes.iter() {
        for ingredient in recipe.ingredients.ingredients.iter() {
            let phrase;
            if ingredient.0 != "" {
                phrase = ingredient.0.clone() + " " + ingredient.1.as_str();
            } else {
                phrase = ingredient.1.clone();
            }
            for word in phrase.as_str().split_ascii_whitespace() {
                println!("{}", word);
            }
            println!();
        }
    }
    */

    /*
    let ingredients = parse_ingredients(&recipes);
        let (recipe_vecs, table, recipe_ingredient, ingredient_cooccurrence) =
            init(recipes, ingredients);

        let mut file = File::open(
            "/home/austen/Documents/school/research/graph-embed/build/examples/temp/part.temp",
        )
        .unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut agg_iter = contents.as_str().split('\n');

        // Meta data about partitioning is on first two lines of partition file
        let mut nk = agg_iter
            .next()
            .expect("Partition file is empty")
            .split_ascii_whitespace();

        // number of vertices before coarsening
        let n: i32 = nk.next().unwrap().parse().unwrap();

        // number of hierarchy levels
        let k: i32 = nk.next().unwrap().parse().unwrap();
        println!("unique ingredients: {}\nlevels: {}", n, k);

        // nodes at each level
        let counts: Vec<i32> = agg_iter
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // the partitioned data
        let mut hierarchies: Vec<Vec<Vec<i32>>> = Vec::new();
        for size in counts {
            let mut level = Vec::new();
            for _ in 0..size {
                let agg = agg_iter
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                level.push(agg);
            }
            hierarchies.push(level);
        }

        for agg in hierarchies[0].iter() {
            for node in agg {
                print!(
                    "{}, ",
                    table
                        .ingredients_vec
                        .get(*node)
                        .unwrap_or(&String::from("err"))
                );
            }
            println!();
            println!();
        }

    ////////
        let table = Table::new(recipes, ingredients);

        println!("{}", table.ingredients_vec.len());
        println!("{}", table.recipes.len());
        println!("{}", table.points.len());

        let mut sorted: Vec<(String, i32)> = table
            .ingredients_vec
            .iter()
            .zip(table.ingredients_count.iter())
            .map(|(i, &c)| (i.clone(), c))
            .collect();

        sorted.sort_by(|(_, a), (_, b)| b.cmp(&a));

        for (i, c) in sorted.iter().take(500) {
            println!("{} {}", i, c)
        }

        for (recipe, ingredients) in recipes.iter().zip(ingredients).take(10) {
            println!("{}", recipe.title);
            for ingredient in ingredients {
                if let Some(quantity) = ingredient.quantity {
                    print!("{}   ", quantity);
                }
                if let Some(measurement) = ingredient.measurement {
                    print!("{}   ", measurement);
                }

                print!("{}\n", ingredient.name);
            }
        }
        */
}
