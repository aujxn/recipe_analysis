use crate::models::Recipe;
use crate::process::Ingredient;
use indexmap::IndexSet;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct RecipeVecs {
    // First index is recipe ID, nested Vec is list of ingredient ID's
    pub recipes: Vec<Vec<usize>>,
}

pub struct Table {
    // Vec of the recipes in the format from database. Index is ID
    pub recipes: Vec<Recipe>,

    // Every unique ingredient, index is ID, value is name
    pub ingredients_vec: Vec<String>,

    // Map to look up ID for an ingredient
    pub ingredients: HashMap<String, usize>,

    // Number of times each ingredient appears
    pub ingredients_count: Vec<usize>,
}

pub struct RecipeIngredient {
    // Every recipe - ingredient co-occurrence
    pub points: Vec<(usize, usize)>,
    // Bipartite sparse adjacency matrix from points
    //pub recipe_ingredient: SparseMatrix<u64>,
}

pub struct IngredientCooccurrence {
    // (ID, ID, count) to build adjacency matrix
    pub points: Vec<(usize, usize, u64)>,
    // Ingredient co-occurrence count adjacency matrix
    //pub ingredient_ingredient: SparseMatrix<u64>,
}

impl IngredientCooccurrence {
    pub fn make_coolist(&self) {
        let coolist = self
            .points
            .iter()
            .map(|(x, y, val)| format!("{} {} {}", x, y, val))
            .join("\n");

        let path = Path::new("temp/coolist");
        let mut temp_file = File::create(&path).unwrap();
        temp_file.write_all(coolist.as_bytes()).unwrap();
    }
}

// ID lookup for ingredient, ingredient lookup from ID, ingredient counts
pub fn ingredient_map(
    ingredients: &Vec<IndexSet<String>>,
) -> (
    HashMap<String, usize>,
    Vec<String>,
    Vec<usize>,
    IngredientCooccurrence,
    Vec<Vec<usize>>,
) {
    let mut ingredient_id = 0;
    let mut recipe_id = 0;
    let recipe_count = ingredients.len();

    // List of all unique ingredients --- indexed by ingredient ID
    let mut ingredients_vec: Vec<String> = vec![];
    // Occurence counts of the ingredients in the above vec
    let mut ingredients_count: Vec<usize> = vec![];
    // Lookup table for ingredient ID given ingredient name
    let mut ingredients_map = HashMap::new();
    // Vec or recipes --- each recipe is a vec of ingredient IDs
    let mut recipe_vecs = vec![vec![]; recipe_count];

    // Go through recipes and create Vec of unique ingredients
    // and Vecs recipes
    for recipe in ingredients {
        for ingredient in recipe {
            match ingredients_map.get(ingredient) {
                Some(&id) => {
                    ingredients_count[id] += 1;
                    recipe_vecs[recipe_id].push(id);
                }
                None => {
                    ingredients_map.insert(ingredient.clone(), ingredient_id);
                    ingredients_vec.push(ingredient.clone());
                    ingredients_count.push(1);
                    recipe_vecs[recipe_id].push(ingredient_id);
                    ingredient_id += 1;
                }
            }
        }
        recipe_id += 1;
    }
    let ingredient_count = ingredients_vec.len();

    let mut ingredient_ingredient_dense: Vec<Vec<u64>> =
        (0..ingredient_count).map(|id| vec![0; id + 1]).collect();
    // Remove duplicate ingredients from recipes TODO: keep track of ammounts
    // Create recipe-ingredient points
    // Create ingredient-ingredients points using the dense matrix
    for recipe_id in 0..recipe_count {
        recipe_vecs[recipe_id].sort();
        recipe_vecs[recipe_id].dedup();
        for (id_1, id_2) in recipe_vecs[recipe_id].iter().tuple_combinations() {
            ingredient_ingredient_dense[*id_2][*id_1] += 1;
        }
    }

    let mut ingredient_ingredient_points = vec![];
    for x in 0..ingredient_count {
        //fix this to include syntax
        for y in 0..x + 1 {
            let count = ingredient_ingredient_dense[x][y];
            if count != 0 {
                ingredient_ingredient_points.push((x, y, count));
            }
        }
    }
    let ic = IngredientCooccurrence {
        points: ingredient_ingredient_points,
    };
    (
        ingredients_map,
        ingredients_vec,
        ingredients_count,
        ic,
        recipe_vecs,
    )
}

pub fn init(
    recipes: Vec<Recipe>,
    ingredients: Vec<Vec<Ingredient>>,
) -> (RecipeVecs, Table, RecipeIngredient, IngredientCooccurrence) {
    assert_eq!(recipes.len(), ingredients.len());
    let recipe_count = recipes.len();

    // Counters to keep track of each ingredient and recipe IDs
    let mut ingredient_id = 0;
    let mut recipe_id = 0;

    let mut ingredients_vec: Vec<String> = vec![];
    let mut ingredients_count: Vec<usize> = vec![];
    let mut ingredients_map = HashMap::new();
    let mut recipe_vecs = vec![vec![]; recipe_count];
    let mut recipe_ingredient_points = vec![];

    // Go through recipes and create Vec of unique ingredients
    // and Vecs recipes
    for recipe in ingredients {
        for ingredient in recipe {
            match ingredients_map.get(&ingredient.name) {
                Some(&id) => {
                    ingredients_count[id] += 1;
                    recipe_vecs[recipe_id].push(id);
                }
                None => {
                    ingredients_map.insert(ingredient.name.clone(), ingredient_id);
                    ingredients_vec.push(ingredient.name.clone());
                    ingredients_count.push(1);
                    recipe_vecs[recipe_id].push(ingredient_id);
                    ingredient_id += 1;
                }
            }
        }
        recipe_id += 1;
    }
    let ingredient_count = ingredients_vec.len();

    let table = Table {
        recipes,
        ingredients_vec,
        ingredients: ingredients_map,
        ingredients_count,
    };

    let mut ingredient_ingredient_dense: Vec<Vec<u64>> =
        (0..ingredient_count).map(|id| vec![0; id + 1]).collect();
    // Remove duplicate ingredients from recipes TODO: keep track of ammounts
    // Create recipe-ingredient points
    // Create ingredient-ingredients points using the dense matrix
    for recipe_id in 0..recipe_count {
        recipe_vecs[recipe_id].sort();
        recipe_vecs[recipe_id].dedup();
        for &ingredient_id in &recipe_vecs[recipe_id] {
            recipe_ingredient_points.push((recipe_id, ingredient_id));
        }
        for (id_1, id_2) in recipe_vecs[recipe_id].iter().tuple_combinations() {
            ingredient_ingredient_dense[*id_2][*id_1] += 1;
        }
    }
    let recipe_vecs = RecipeVecs {
        recipes: recipe_vecs,
    };

    let mut ingredient_ingredient_points = vec![];
    for x in 0..ingredient_count {
        //fix this to include syntax
        for y in 0..x + 1 {
            let count = ingredient_ingredient_dense[x][y];
            if count != 0 {
                ingredient_ingredient_points.push((x, y, count));
            }
        }
    }
    let recipe_ingredient = RecipeIngredient {
        points: recipe_ingredient_points,
    };
    let ingredient_cooccurrence = IngredientCooccurrence {
        points: ingredient_ingredient_points,
    };

    (
        recipe_vecs,
        table,
        recipe_ingredient,
        ingredient_cooccurrence,
    )
}
