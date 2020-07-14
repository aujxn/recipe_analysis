use anyhow::Result;
use matrixlab::matrix::sparse::SparseMatrix;
use matrixlab::MatrixElement;
use std::collections::HashMap;

pub struct Relation {
    // Key: name Value: ID, Count
    ingredient_map: HashMap<String, (usize, usize)>,
    // Indexed on ID
    ingredients_list: Vec<String>,
    // [i, j, val, i, j, val....]
    coolist: Vec<i32>,
    recipe_count: usize,
}

impl Relation {
    pub fn get_ingredient_map(&self) -> &HashMap<String, (usize, usize)> {
        &self.ingredient_map
    }

    pub fn get_ingredient_list(&self) -> &Vec<String> {
        &self.ingredients_list
    }

    pub fn get_coolist(&self) -> &Vec<i32> {
        &self.coolist
    }

    pub fn get_recipe_count(&self) -> usize {
        self.recipe_count
    }

    pub fn into_parts(
        self,
    ) -> (
        HashMap<String, (usize, usize)>,
        Vec<String>,
        Vec<i32>,
        usize,
    ) {
        (
            self.ingredient_map,
            self.ingredients_list,
            self.coolist,
            self.recipe_count,
        )
    }
}

pub async fn make_relation(recipes: Vec<(i32, Vec<String>)>) -> Result<Relation> {
    let mut ingredient_map = HashMap::new();
    let mut ingredients_list = vec![];
    let mut recipe_ingredient = vec![];
    let recipe_count = recipes.len();

    for (i, (_, ingredients)) in recipes.into_iter().enumerate() {
        for ingredient in ingredients {
            let j = match ingredient_map.get_mut(&ingredient) {
                Some((id, mut _count)) => {
                    _count += 1;
                    *id
                }
                None => {
                    let id = ingredients_list.len();
                    ingredients_list.push(ingredient.clone());
                    ingredient_map.insert(ingredient, (id, 0));
                    id
                }
            };
            recipe_ingredient.push(MatrixElement::new(i, j, 1));
        }
    }

    // TODO: fix error in matrixlab so no unwrap
    let recipe_ingredient =
        SparseMatrix::new(recipe_count, ingredients_list.len(), recipe_ingredient).unwrap();

    let ingredient_ingredient = &recipe_ingredient.transpose() * &recipe_ingredient;

    let coolist = ingredient_ingredient
        .elements()
        .filter(|(i, j, _)| i > j)
        .map(|(i, j, val)| vec![i as i32, j as i32, *val as i32])
        .flatten()
        .collect();

    Ok(Relation {
        ingredient_map,
        ingredients_list,
        coolist,
        recipe_count,
    })
}
