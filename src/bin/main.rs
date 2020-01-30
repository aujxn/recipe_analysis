extern crate recipe_scraper;

use recipe_scraper::models::Recipe;
use recipe_scraper::process::*;
use recipe_scraper::table::*;

fn main() {
    let recipes = pull_recipes();
    let ingredients = parse_ingredients(&recipes);

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

    /*
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
