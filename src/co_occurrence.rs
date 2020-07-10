use diesel::prelude::*;
use indexmap::IndexSet;
use itertools::Itertools;
use matrixlab::matrix::sparse::SparseMatrix;
use matrixlab::MatrixElement;
use std::collections::HashMap;
use std::io::prelude::*;
use std::path::Path;

pub fn make_coolist(
    recipes: Vec<(i32, String)>,
    target_ingredients: Vec<String>,
) -> (usize, Vec<[i32; 3]>, Vec<String>) {
    let mut recipe_map: HashMap<usize, IndexSet<String>> = HashMap::new();
    let mut ingredient_map = HashMap::new();
    let mut ingredient_counter: usize = 0;
    let mut ingredients_vec = vec![];
    let target_ingredients: IndexSet<String> = target_ingredients.into_iter().collect();

    for (recipe_id, ingredient) in recipes.into_iter() {
        match recipe_map.get_mut(&(recipe_id as usize)) {
            Some(ingredient_list) => {
                ingredient_list.insert(ingredient);
            }
            None => {
                let mut map = IndexSet::new();
                map.insert(ingredient);
                recipe_map.insert(recipe_id as usize, map);
            }
        }
    }

    let mut points = vec![];

    for (i, (_, ingredients)) in recipe_map
        .iter()
        .filter(|(_, ingredients)| target_ingredients.is_subset(ingredients))
        .enumerate()
    {
        for ingredient in ingredients {
            let j = match ingredient_map.get(&ingredient) {
                Some(&id) => id,
                None => {
                    let id = ingredient_counter;
                    ingredients_vec.push(ingredient.clone());
                    ingredient_map.insert(ingredient, ingredient_counter);
                    ingredient_counter += 1;
                    id
                }
            };
            points.push(MatrixElement::new(i, j, 1));
        }
    }

    let recipe_count = recipe_map.len();
    let recipe_ingredient = SparseMatrix::new(recipe_count, ingredient_counter, points).unwrap();

    let ingredient_ingredient = &recipe_ingredient.transpose() * &recipe_ingredient;

    assert_eq!(
        ingredient_ingredient.num_rows(),
        ingredient_ingredient.num_columns()
    );

    let coolist = ingredient_ingredient
        .elements()
        .filter(|(i, j, _)| i > j)
        .map(|(i, j, val)| [i as i32, j as i32, *val as i32])
        .collect();

    (recipe_count, coolist, ingredients_vec)
}
