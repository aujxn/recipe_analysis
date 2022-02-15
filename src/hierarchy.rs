use crate::expanded::ExpandedIngredientRelation;
use anyhow::{anyhow, Result};
use matrixlab::matrix::sparse::SparseMatrix;
use std::collections::BTreeMap;

pub struct Hierarchy {
    interpolation_matrices: Vec<SparseMatrix<usize>>,
    ingredients_vec: Vec<String>,
    ingredients_map: BTreeMap<String, (usize, usize)>,
    ingredient_ingredient: SparseMatrix<usize>,
    relation: ExpandedIngredientRelation,
}

impl Hierarchy {
    pub async fn new(
        interpolation_matrices: Vec<SparseMatrix<usize>>,
        ingredients_vec: Vec<String>,
        ingredients_map: BTreeMap<String, (usize, usize)>,
        relation: ExpandedIngredientRelation,
    ) -> Hierarchy {
        let ingredient_ingredient = relation.build_adjacency_matrix().await;
        Self {
            interpolation_matrices,
            ingredients_map,
            ingredients_vec,
            ingredient_ingredient,
            relation,
        }
    }

    pub fn ingredients_map(&self) -> &BTreeMap<String, (usize, usize)> {
        &self.ingredients_map
    }

    pub fn adjacency_matrix(&self) -> &SparseMatrix<usize> {
        &self.ingredient_ingredient
    }

    pub fn num_levels(&self) -> usize {
        self.interpolation_matrices.len() + 1
    }

    pub fn generate_recipes(
        &self,
        level: usize,
        target_ingredients: Vec<String>,
    ) -> Result<Vec<Vec<(String, usize)>>> {
        if level > self.interpolation_matrices.len() {
            Err(anyhow!("out of range"))
        } else {
            let mut partition = self.interpolation_matrices[0].clone();
            for i in 1..level {
                partition = &partition * &self.interpolation_matrices[i];
            }

            let mut aggregates: Vec<BTreeMap<String, Vec<usize>>> =
                vec![BTreeMap::new(); partition.num_columns()];

            for (node, agg, _) in partition.elements() {
                let ingredient_id = self.relation.get_ingredient_id(node);
                let recipe_id = self.relation.get_recipe_id(node);

                if ingredient_id.is_none() || recipe_id.is_none() {
                    continue;
                }
                let ingredient_id = ingredient_id.unwrap();
                let recipe_id = recipe_id.unwrap();

                let name = &self.ingredients_vec[ingredient_id];
                match aggregates[agg].get_mut(name) {
                    Some(recipes) => {
                        recipes.push(recipe_id);
                    }
                    None => {
                        aggregates[agg].insert(name.clone(), vec![recipe_id]);
                    }
                };
            }

            let filtered: Vec<BTreeMap<String, Vec<usize>>> = aggregates
                .into_iter()
                .filter(|ingredients| {
                    target_ingredients
                        .iter()
                        .all(|target| ingredients.contains_key(target))
                })
                .collect();

            let sorted_ingredients: Vec<Vec<(String, usize)>> = filtered
                .iter()
                .map(|ingredients| {
                    let mut ingredients: Vec<(String, usize)> = ingredients
                        .iter()
                        .map(|(x, y)| (x.clone(), y.len()))
                        .collect();
                    ingredients.sort_unstable_by(|a, b| a.1.cmp(&b.1));
                    ingredients
                })
                .collect();
            Ok(sorted_ingredients)
        }
    }
}
