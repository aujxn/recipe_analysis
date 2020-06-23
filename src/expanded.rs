use itertools::Itertools;
use matrixlab::matrix::sparse::SparseMatrix;
use matrixlab::MatrixElement;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

enum ExpandedVertex {
    // has only an associated ingredient ID
    IngredientHub(usize),
    // has only an associated recipe ID
    RecipeHub(usize),

    // A Vertex has an associated ingredient and recipe ID
    Vertex((usize, usize)),
}

impl ExpandedVertex {
    pub fn get_ingredient_id(&self) -> Option<usize> {
        match self {
            Self::IngredientHub(id) => Some(*id),
            Self::Vertex((id, _)) => Some(*id),
            Self::RecipeHub(_) => None,
        }
    }

    pub fn get_recipe_id(&self) -> Option<usize> {
        match self {
            Self::IngredientHub(id) => None,
            Self::Vertex((_, id)) => Some(*id),
            Self::RecipeHub(id) => Some(*id),
        }
    }
}

pub struct ExpandedIngredientRelation {
    vertices: Vec<ExpandedVertex>,
    edges: Vec<(usize, usize)>,
}

impl ExpandedIngredientRelation {
    pub fn new(recipes: Vec<Vec<usize>>, num_ingredients: usize) -> ExpandedIngredientRelation {
        let mut vertices: Vec<ExpandedVertex> = (0..num_ingredients)
            .map(|id| ExpandedVertex::IngredientHub(id))
            .collect();
        let mut edges: Vec<(usize, usize)> = vec![];
        let mut counter = num_ingredients;

        println!(
            "num_ingredients: {}\n num_recipes: {}",
            num_ingredients,
            recipes.len()
        );

        for (recipe_id, recipe) in recipes.iter().enumerate() {
            vertices.push(ExpandedVertex::RecipeHub(recipe_id));
            let recipe_index = counter;
            counter += 1;
            for &ingredient_id in recipe {
                vertices.push(ExpandedVertex::Vertex((ingredient_id, recipe_id)));
                edges.push((ingredient_id, counter));
                edges.push((recipe_index, counter));
                counter += 1;
            }
        }

        ExpandedIngredientRelation { vertices, edges }
    }

    pub fn connect_clique(&mut self, ingredients: &Vec<usize>) {
        let target_vertices: Vec<usize> = self
            .vertices
            .iter()
            .enumerate()
            .filter_map(|(i, vertex)| {
                if ingredients.iter().any(|target| {
                    if let Some(id) = vertex.get_ingredient_id() {
                        *target == id
                    } else {
                        false
                    }
                }) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        let target_index = self.vertices.len();
        self.vertices.push(ExpandedVertex::RecipeHub(9999999));
        for index in target_vertices {
            self.edges.push((index, target_index));
        }
        /*
        for pair in target_vertices.into_iter().tuple_combinations() {
            self.edges.push(pair);
        }

        self.edges.sort();
        self.edges.dedup();
        */
    }

    pub fn get_ingredient_id(&self, node: usize) -> Option<usize> {
        self.vertices[node].get_ingredient_id()
    }

    pub fn get_recipe_id(&self, node: usize) -> Option<usize> {
        self.vertices[node].get_recipe_id()
    }

    pub fn number_of_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn number_of_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn build_coolist(&self) {
        let coolist = self
            .edges
            .iter()
            .map(|(x, y)| format!("{} {}", x, y))
            .join("\n");

        let path = Path::new("temp/expanded_coolist");
        let mut temp_file = File::create(&path).unwrap();
        temp_file.write_all(coolist.as_bytes()).unwrap();
    }

    pub fn build_adjacency_matrix(&self) -> SparseMatrix<usize> {
        let matrix_elements: Vec<MatrixElement<usize>> = self
            .edges
            .iter()
            .map(|(i, j)| {
                vec![
                    MatrixElement::new(*i, *j, 1usize),
                    MatrixElement::new(*j, *i, 1usize),
                ]
            })
            .flatten()
            .collect();

        let number_of_vertices = self.number_of_vertices();
        SparseMatrix::new(number_of_vertices, number_of_vertices, matrix_elements).unwrap()
    }
}
