use itertools::Itertools;
use matrixlab::matrix::sparse::SparseMatrix;
use matrixlab::MatrixElement;
use std::collections::BTreeMap;
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
            Self::IngredientHub(_) => None,
            Self::Vertex((_, id)) => Some(*id),
            Self::RecipeHub(id) => Some(*id),
        }
    }
}

pub struct ExpandedIngredientRelation {
    vertices: Vec<ExpandedVertex>,
    edges: BTreeMap<(usize, usize), usize>,
}

impl ExpandedIngredientRelation {
    /// Creates an expanded ingredient relation based on stars with internal nodes
    /// for recipes and ingredients. Each ingredients in the list of recipes is connected
    /// to the associated internal node for recipe and ingredients. An internal node for
    /// the target ingredients is added and all of the ingredient nodes that are in the
    /// target ingredients set are connected to this target internal node. The result is
    /// a graph of overlapping stars where ingredient vertices that are not part of the
    /// target set have a degree of two, ingredient vertices that are part of the target
    /// set have a degree of three, recipe internal nodes have a degree equal to the number
    /// of ingredients in the recipe, ingredient internal nodes have degree equal to the
    /// number of that ingredient, and the target internal node has degree equal to the
    /// number of ingredients in all the recipes that are in the target set.
    pub fn build_stars(
        recipes: Vec<Vec<usize>>,
        num_ingredients: usize,
    ) -> ExpandedIngredientRelation {
        let mut vertices: Vec<ExpandedVertex> = (0..num_ingredients)
            .map(|id| ExpandedVertex::IngredientHub(id))
            .collect();
        let mut edges: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        let mut counter = num_ingredients;

        for (recipe_id, recipe) in recipes.iter().enumerate() {
            vertices.push(ExpandedVertex::RecipeHub(recipe_id));
            let recipe_index = counter;
            counter += 1;
            for &ingredient_id in recipe {
                vertices.push(ExpandedVertex::Vertex((ingredient_id, recipe_id)));
                edges.insert((ingredient_id, counter), 1);
                edges.insert((recipe_index, counter), 1);
                counter += 1;
            }
        }

        ExpandedIngredientRelation { vertices, edges }
    }

    pub fn build_cliques(
        recipes: Vec<Vec<usize>>,
        target_ingredients: Vec<usize>,
        num_ingredients: usize,
    ) -> ExpandedIngredientRelation {
        // Vec of indices where each ingredient can be found in the vertices vec
        let mut ingredient_vertices: Vec<Vec<usize>> = vec![vec![]; num_ingredients];
        // Each ingredient of each recipe makes a vertex
        let mut vertices: Vec<ExpandedVertex> = vec![];
        // Key: (i, j) of the vertex (corresponding indices from vertices vec), Value: weight
        let mut edges: BTreeMap<(usize, usize), usize> = BTreeMap::new();

        // Creates a vertex for each ingredient in every recipe and adds edges between
        // all vertices from the same recipe
        for (recipe_id, ingredients) in recipes.iter().enumerate() {
            let start = vertices.len();
            let end = start + ingredients.len();
            // Add each ingredient from one recipe into the vertices vec
            for ingredient_id in ingredients.iter() {
                ingredient_vertices[*ingredient_id].push(vertices.len());
                vertices.push(ExpandedVertex::Vertex((*ingredient_id, recipe_id)));
            }
            // Add edges between each vertex from one recipe
            for (i, j) in (start..end).tuple_combinations() {
                edges.insert((i, j), 1);
            }
        }

        // Add edges between each vertex representing the same ingredient
        for ingredient in ingredient_vertices.iter() {
            for (i, j) in ingredient.iter().tuple_combinations() {
                edges.insert((*i, *j), 1);
            }
        }

        // Add edges between each vertex representing ingredients from the target ingredient set
        for (i, j) in target_ingredients.iter().tuple_combinations() {
            for first_ingredient in ingredient_vertices[*i].iter() {
                for second_ingredient in ingredient_vertices[*j].iter() {
                    if let Some(weight) = edges.get_mut(&(*first_ingredient, *second_ingredient)) {
                        *weight += 1;
                    } else {
                        edges.insert((*first_ingredient, *second_ingredient), 1);
                    }
                }
            }
        }

        // check that i < j always
        for ((i, j), _) in edges.iter() {
            assert!(i < j);
        }

        ExpandedIngredientRelation { vertices, edges }
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
            .map(|((i, j), weight)| format!("{} {} {}", i, j, weight))
            .join("\n");

        let path = Path::new("temp/expanded_coolist");
        let mut temp_file = File::create(&path).unwrap();
        temp_file.write_all(coolist.as_bytes()).unwrap();
    }

    pub fn build_adjacency_matrix(&self) -> SparseMatrix<usize> {
        let matrix_elements: Vec<MatrixElement<usize>> = self
            .edges
            .iter()
            .map(|((i, j), weight)| {
                vec![
                    MatrixElement::new(*i, *j, *weight),
                    MatrixElement::new(*j, *i, *weight),
                ]
            })
            .flatten()
            .collect();

        let number_of_vertices = self.number_of_vertices();
        SparseMatrix::new(number_of_vertices, number_of_vertices, matrix_elements).unwrap()
    }
}
