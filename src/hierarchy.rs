use crate::expanded::ExpandedIngredientRelation;
use indexmap::set::IndexSet;
use matrixlab::matrix::sparse::SparseMatrix;
use std::collections::HashMap;

pub struct Hierarchy {
    interpolation_matrices: Vec<SparseMatrix<usize>>,
    ingredients_vec: Vec<String>,
    ingredients_map: HashMap<String, usize>,
    ingredient_ingredient: SparseMatrix<usize>,
    relation: ExpandedIngredientRelation,
}

impl Hierarchy {
    pub fn new(
        interpolation_matrices: Vec<SparseMatrix<usize>>,
        ingredients_vec: Vec<String>,
        ingredients_map: HashMap<String, usize>,
        relation: ExpandedIngredientRelation,
    ) -> Hierarchy {
        let ingredient_ingredient = relation.build_adjacency_matrix();
        Self {
            interpolation_matrices,
            ingredients_map,
            ingredients_vec,
            ingredient_ingredient,
            relation,
        }
    }

    pub fn num_levels(&self) -> usize {
        self.interpolation_matrices.len() + 1
    }

    pub fn generate_recipes(
        &self,
        level: usize,
    ) -> Result<Vec<HashMap<String, IndexSet<usize>>>, &'static str> {
        if level > self.interpolation_matrices.len() {
            Err("out of range")
        } else {
            let mut partition = self.interpolation_matrices[0].clone();
            for i in 1..level {
                partition = &partition * &self.interpolation_matrices[i];
            }

            let mut aggregates: Vec<HashMap<String, IndexSet<usize>>> =
                vec![HashMap::new(); partition.num_columns()];

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
                        recipes.insert(recipe_id);
                    }
                    None => {
                        let mut set = IndexSet::new();
                        set.insert(recipe_id);
                        aggregates[agg].insert(name.clone(), set);
                    }
                };
            }
            Ok(aggregates)
        }
    }
}
