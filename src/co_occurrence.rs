use anyhow::Result;
use matrixlab::matrix::sparse::SparseMatrix;
use matrixlab::MatrixElement;
use std::collections::BTreeMap;

pub struct Relation {
    // Key: name Value: ID, Count
    ingredient_map: BTreeMap<String, (usize, usize)>,
    // Indexed on ID
    ingredients_list: Vec<String>,
    ingredient_ingredient: SparseMatrix<usize>,
    recipe_count: usize,
}

impl Relation {
    pub fn get_ingredient_map(&self) -> &BTreeMap<String, (usize, usize)> {
        &self.ingredient_map
    }

    pub fn get_ingredient_list(&self) -> &Vec<String> {
        &self.ingredients_list
    }

    pub fn get_matrix(&self) -> &SparseMatrix<usize> {
        &self.ingredient_ingredient
    }

    pub fn get_recipe_count(&self) -> usize {
        self.recipe_count
    }

    pub fn into_parts(
        self,
    ) -> (
        BTreeMap<String, (usize, usize)>,
        Vec<String>,
        SparseMatrix<usize>,
        usize,
    ) {
        (
            self.ingredient_map,
            self.ingredients_list,
            self.ingredient_ingredient,
            self.recipe_count,
        )
    }

    pub fn write_files(&self) {
        use std::fs;
        use std::io::prelude::*;
        let mut file = fs::File::create("temp/ingredient_ingredient.coo").unwrap();

        let coolist: String = self
            .ingredient_ingredient
            .elements()
            .filter(|(i, j, _)| i != j)
            .map(|(i, j, val)| format!("{} {} {}", i, j, val))
            .collect::<Vec<String>>()
            .join("\n");

        file.write_all(coolist.as_bytes()).unwrap();

        let mut file = fs::File::create("temp/ingredient_labels.txt").unwrap();
        let labels: String = self.ingredients_list.join("\n");
        file.write_all(labels.as_bytes()).unwrap();
    }
}

pub async fn recipe_ingredient(
    recipes: &Vec<(i32, Vec<String>)>,
) -> Result<(
    SparseMatrix<usize>,
    BTreeMap<String, (usize, usize)>,
    Vec<String>,
)> {
    let mut ingredient_map = BTreeMap::new();
    let mut ingredients_list = vec![];
    let mut recipe_ingredient = vec![];
    let recipe_count = recipes.len();

    for (i, (_, ingredients)) in recipes.into_iter().enumerate() {
        for ingredient in ingredients.into_iter() {
            let j = match ingredient_map.get_mut(ingredient) {
                Some((id, mut _count)) => {
                    _count += 1;
                    *id
                }
                None => {
                    let id = ingredients_list.len();
                    ingredients_list.push(ingredient.clone());
                    ingredient_map.insert(ingredient.clone(), (id, 1));
                    id
                }
            };
            recipe_ingredient.push(MatrixElement::new(i, j, 1));
        }
    }

    // TODO: fix error in matrixlab so no unwrap
    let recipe_ingredient =
        SparseMatrix::new(recipe_count, ingredients_list.len(), recipe_ingredient).unwrap();

    Ok((recipe_ingredient, ingredient_map, ingredients_list))
}

pub async fn make_relation(recipes: &Vec<(i32, Vec<String>)>) -> Result<Relation> {
    let recipe_count = recipes.len();
    let (recipe_ingredient, ingredient_map, ingredients_list) = recipe_ingredient(&recipes).await?;
    let ingredient_ingredient = &recipe_ingredient.transpose() * &recipe_ingredient;

    Ok(Relation {
        ingredient_map,
        ingredients_list,
        ingredient_ingredient,
        recipe_count,
    })
}
