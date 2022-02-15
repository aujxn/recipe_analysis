use anyhow::Result;
use recipe_analysis::{co_occurrence, expanded, hierarchy, louvain, recipe};
use std::fs;
use std::io::prelude::*;
use std::io::{self, Write};
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

//use recipe_analysis::graph_explorer;

#[tokio::main]
async fn main() -> Result<()> {
    let target_ingredients = vec!["steak", "tofu"];
    let target_ingredients = target_ingredients
        .into_iter()
        .map(|name| String::from(name))
        .collect();
    println!("querying recipes");
    let recipes = recipe::query_filtered_recipes(None, None, Some(target_ingredients)).await?;

    /*
    println!("querying recipes");
    let recipes =
        recipe::query_filtered_recipes(Some("Salads And Dressings".into()), None, None).await?;
    */

    let num_recipes = recipes.len();
    println!(
        "{} recipes included...\nbuilding co_occurrence",
        num_recipes
    );

    let relation = co_occurrence::make_relation(&recipes).await.unwrap();
    relation.write_files();

    let (_, _, matrix, _) = relation.into_parts();
    let matrix_string = recipe_analysis::export(&matrix);

    let mut child = Command::new("../graph-embed/build/examples/embed")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(matrix_string.as_bytes()).await?;
    drop(child_stdin);

    let output = child.wait_with_output().await?;
    let output = String::from_utf8(output.stdout).unwrap();

    let mut file = fs::File::create("temp/embedding.txt").unwrap();
    file.write_all(output.as_bytes()).unwrap();
    /*
    let partition = graph_explorer::load_partfile();
    let coords = graph_explorer::load_coords();
    let embedding = graph_explorer::Embedding::new(coords, partition, matrix, ingredient_list);

    graph_explorer::run(embedding);
    */

    /*
    let (recipe_ingredient, ingredient_map, ingredients_list) =
        co_occurrence::recipe_ingredient(&recipes).await?;
    let recipes = recipe_ingredient
        .row_iter()
        .map(|(_, ingredients)| ingredients.iter().map(|x| *x).collect())
        .collect();
    let target_ingredients_ids = target_ingredients
        .iter()
        .map(|name| ingredient_map.get(name).unwrap().0)
        .collect();
    let num_ingredients = ingredients_list.len();

    println!("building expanded relation");
    let expanded_relation = expanded::ExpandedIngredientRelation::build_stars(
        recipes,
        target_ingredients_ids,
        num_ingredients,
    )
    .await;

    expanded_relation.build_coolist().await;
    let nnv = expanded_relation.number_of_vertices();
    println!("louvains - number of vetices: {}", nnv);
    let interpolation_matrices = louvain::louvain(expanded_relation.number_of_vertices()).await;

    let hierarchy = hierarchy::Hierarchy::new(
        interpolation_matrices,
        ingredients_list,
        ingredient_map,
        expanded_relation,
    )
    .await;

    println!("number of levels: {}", hierarchy.num_levels());
    let recipes = hierarchy.generate_recipes(2, target_ingredients)?;

    for recipe in recipes.iter() {
        for (ingredient, count) in recipe.iter() {
            println!("{} {}", ingredient, count);
        }
        println!();
    }
    */

    Ok(())
}
