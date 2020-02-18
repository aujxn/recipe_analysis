extern crate recipe_analysis;

//use recipe_analysis::models::Recipe;
use recipe_analysis::models::NytcQueryable;
use recipe_analysis::nytc::Nytc;
//use recipe_analysis::process::*;
use recipe_analysis::process_ny::*;
use recipe_analysis::table::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut recipes = pull_recipes();

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

    for recipe in recipes.iter().take(5) {
        recipe.display();
    }

    println!("count: {}\n", recipes.len());

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
        let n: usize = nk.next().unwrap().parse().unwrap();

        // number of hierarchy levels
        let k: usize = nk.next().unwrap().parse().unwrap();
        println!("unique ingredients: {}\nlevels: {}", n, k);

        // nodes at each level
        let counts: Vec<usize> = agg_iter
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // the partitioned data
        let mut hierarchies: Vec<Vec<Vec<usize>>> = Vec::new();
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

        let mut sorted: Vec<(String, usize)> = table
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
