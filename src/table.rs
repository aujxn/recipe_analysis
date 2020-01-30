use crate::models::Recipe;
use crate::process::Ingredient;
use matrixlab::matrix::sparse::SparseMatrix;
use matrixlab::MatrixElement;
use std::collections::HashMap;

pub struct Table {
    pub recipes: Vec<Recipe>,
    pub ingredients: HashMap<String, usize>,
    pub ingredients_vec: Vec<String>,
    pub ingredients_count: Vec<usize>,
    pub points: Vec<(usize, usize)>,
    pub recipe_ingredient: SparseMatrix<u64>,
}

impl Table {
    pub fn new(recipes: Vec<Recipe>, ingredients: Vec<Vec<Ingredient>>) -> Self {
        let mut ingredient_id = 0;
        let mut recipe_id = 0;
        let mut points = vec![];
        let mut ingredients_vec: Vec<String> = vec![];
        let mut ingredients_count: Vec<usize> = vec![];
        let mut ingredients_map = HashMap::new();
        for recipe in ingredients {
            recipe_id += 1;
            for ingredient in recipe {
                match ingredients_map.get(&ingredient.name) {
                    Some(&id) => {
                        points.push((recipe_id, id));
                        ingredients_count[id] += 1;
                    }
                    None => {
                        ingredients_map.insert(ingredient.name.clone(), ingredient_id);
                        ingredients_vec.push(ingredient.name.clone());
                        ingredients_count.push(1);
                        points.push((recipe_id, ingredient_id));
                        ingredient_id += 1;
                    }
                }
            }
        }
        let mut elements = points
            .iter()
            .map(|(recipe, ingredient)| MatrixElement::new(*recipe, *ingredient, 1));
        let mut recipe_ingredient =
            SparseMatrix::new(recipes.len(), ingredients_vec.len(), elements.collect()).unwrap();
        Table {
            recipes,
            ingredients: ingredients_map,
            ingredients_vec,
            ingredients_count,
            points,
            recipe_ingredient,
        }
    }
}
