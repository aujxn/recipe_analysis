use super::schema::recipe_ingredient;
use bimap::BiMap;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use indexmap::IndexSet;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug)]
struct Recipe {
    id: usize,
    title: String,
    source: Publisher,
    url: String,
    yields: String,
    /// Prep + Cooking time in minutes
    time: Option<usize>,
    description: Option<String>,
    tags: Option<IndexSet<usize>>,
    num_ratings: usize,
    avg_rating: f32,
    ingredients: IngredientsList,
    steps: Vec<String>,
    comments: Option<Vec<(String, usize)>>,
}

struct Comment {
    id: usize,
    recipes_id: usize,
    body: String,
    votes: usize,
}

struct QueryableRecipe {
    id: usize,
    title: String,
    source: Publisher,
    url: String,
    yields: String,
    time: Option<usize>,
    description: Option<String>,
    steps: Vec<String>,
    num_ratings: usize,
    avg_rating: f32,
}

#[derive(Debug)]
enum Publisher {
    /// New York Times Cooking
    NYTCooking,
    /// allrecipes.com
    AllRecipes,
}

#[derive(Debug, DbEnum)]
#[PgType = "measurement"]
#[DieselType = "Measurement"]
pub enum Quantity {
    /// # of grams
    Mass,
    /// # of milliliters
    Volume,
}

#[derive(Queryable, Debug)]
pub struct RecipeIngredient {
    recipes_id: usize,
    ingredient_id: usize,
    sub_components_id: Option<usize>,
    quantity_type: Option<Quantity>,
    quantity_value: Option<usize>,
    quantity_note: Option<String>,
}

#[derive(Debug)]
enum IngredientsList {
    /// Vec of component IDs and associated ingredient IDs with quantity
    HasSubComponents(Vec<(usize, Vec<(usize, Quantity)>)>),
    /// Vec of ingredient IDs with quantity
    NoSubComponents(Vec<(usize, Quantity)>),
}

/*
#[derive(Debug)]
enum Quantity {
    /// # of grams
    Mass(usize),
    /// # of milliliters
    Volume(usize),
}
*/

struct Ingredients {
    /// Bijective map to look up ingredient names by ID or IDs by name
    ingredients: BiMap<usize, String>,
    /// Bijective map to look up component names by ID or IDs by name
    components: BiMap<usize, String>,
}

struct Recipes {
    /// Lookup recipes by ID
    recipes: HashMap<usize, Recipe>,
}
