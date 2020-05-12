use itertools::Itertools;
use matrixlab::matrix::sparse::SparseMatrix;
use matrixlab::MatrixElement;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

enum ExpandedVertex {
    // A Node has only an associated ingredient ID
    Node(usize),
    // A Vertex has an associated ingredient and recipe ID
    Vertex((usize, usize)),
}

impl ExpandedVertex {
    pub fn get_ingredient_id(&self) -> usize {
        match self {
            Self::Node(id) => *id,
            Self::Vertex((id, _)) => *id,
        }
    }

    pub fn get_recipe_id(&self) -> Option<usize> {
        match self {
            Self::Node(id) => None,
            Self::Vertex((_, id)) => Some(*id),
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
            .map(|id| ExpandedVertex::Node(id))
            .collect();
        let mut edges: Vec<(usize, usize)> = vec![];
        let mut counter = num_ingredients;

        println!(
            "num_ingredients: {}\n num_recipes: {}",
            num_ingredients,
            recipes.len()
        );

        for (recipe_id, recipe) in recipes.iter().enumerate() {
            let start = counter;
            for &ingredient_id in recipe {
                vertices.push(ExpandedVertex::Vertex((ingredient_id, recipe_id)));
                edges.push((ingredient_id, counter));
                counter += 1;
            }
            for pair in (start..counter).tuple_combinations() {
                edges.push(pair);
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
                if ingredients
                    .iter()
                    .any(|target| *target == vertex.get_ingredient_id())
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        for pair in target_vertices.into_iter().tuple_combinations() {
            self.edges.push(pair);
        }

        self.edges.sort();
        self.edges.dedup();
    }

    pub fn get_ingredient_id(&self, node: usize) -> usize {
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
