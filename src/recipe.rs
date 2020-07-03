use super::schema::{
    comments, ingredients, recipe_ingredient, recipe_tag, recipes, sub_components, tags,
};
use anyhow::{anyhow, Result};
use bimap::BiMap;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
struct Recipe {
    id: Option<usize>,
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

#[derive(Queryable, Identifiable, Associations, Debug, Clone, Serialize)]
#[belongs_to(QueryableRecipe, foreign_key = "recipes_id")]
#[table_name = "comments"]
struct Comment {
    id: i32,
    recipes_id: i32,
    body: String,
    votes: i32,
}

// TODO port comments from nyt
#[derive(Insertable, Debug, Clone, Serialize)]
#[table_name = "comments"]
struct NewComment {
    recipes_id: i32,
    body: String,
    votes: i32,
}

#[derive(Queryable, Identifiable, Debug, Clone, Serialize)]
#[table_name = "ingredients"]
struct Ingredient {
    id: i32,
    name: String,
}

#[derive(Insertable, Debug, Clone, Serialize)]
#[table_name = "ingredients"]
struct NewIngredient {
    name: String,
}

#[derive(Queryable, Identifiable, Debug, Clone, Serialize)]
#[table_name = "recipe_ingredient"]
struct RecipeIngredient {
    id: i32,
    recipes_id: i32,
    ingredients_id: i32,
    sub_components_id: Option<i32>,
    quantity: f32,
    quantity_note: Option<String>,
}

// TODO port sub_components and note from nyt
#[derive(Insertable, Debug, Clone, Serialize)]
#[table_name = "recipe_ingredient"]
struct NewRecipeIngredient {
    recipes_id: i32,
    ingredients_id: i32,
    sub_components_id: Option<i32>,
    quantity: f32,
    quantity_note: Option<String>,
}

#[derive(Queryable, Identifiable, Debug, Clone, Serialize)]
#[table_name = "recipe_tag"]
struct RecipeTag {
    id: i32,
    recipes_id: i32,
    tags_id: i32,
}

#[derive(Insertable, Debug, Clone, Serialize)]
#[table_name = "recipe_tag"]
struct NewRecipeTag {
    recipes_id: i32,
    tags_id: i32,
}

#[derive(Queryable, Identifiable, Debug, Clone, Serialize)]
#[table_name = "tags"]
struct Tag {
    id: i32,
    name: String,
}

#[derive(Insertable, Debug, Clone, Serialize)]
#[table_name = "tags"]
struct NewTag {
    name: String,
}

#[derive(Queryable, Identifiable, Debug, Clone, Serialize)]
#[table_name = "sub_components"]
struct SubComponent {
    id: i32,
    name: String,
}

#[derive(Insertable, Debug, Clone, Serialize)]
#[table_name = "sub_components"]
struct NewSubComponent {
    name: String,
}

#[derive(Identifiable, Debug, Clone, Serialize)]
#[table_name = "recipes"]
struct QueryableRecipe {
    id: i32,
    title: String,
    source: Publisher,
    url: String,
    yields: String,
    time: Option<i32>,
    description: Option<String>,
    steps: Vec<String>,
    num_ratings: i32,
    avg_rating: f32,
}

// TODO: find out why this doesn't work with derive Queryable macro
type DB = diesel::pg::Pg;
impl Queryable<recipes::SqlType, DB> for QueryableRecipe {
    type Row = (
        i32,
        String,
        i32,
        String,
        String,
        Option<i32>,
        Option<String>,
        Vec<String>,
        i32,
        f32,
    );

    fn build(row: Self::Row) -> Self {
        QueryableRecipe {
            id: row.0,
            title: row.1,
            source: Publisher::new(row.2).unwrap(),
            url: row.3,
            yields: row.4,
            time: row.5,
            description: row.6,
            steps: row.7,
            num_ratings: row.8,
            avg_rating: row.9,
        }
    }
}

#[derive(Insertable, Debug, Clone, Serialize)]
#[table_name = "recipes"]
struct NewRecipe {
    title: String,
    source: Publisher,
    url: String,
    yields: String,
    time: Option<i32>,
    description: Option<String>,
    steps: Vec<String>,
    num_ratings: i32,
    avg_rating: f32,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, Serialize, AsExpression)]
#[sql_type = "Integer"]
enum Publisher {
    /// New York Times Cooking
    NYTCooking = 0,
    /// allrecipes.com
    AllRecipes = 1,
}

impl Publisher {
    fn new(value: i32) -> Result<Self> {
        match value {
            0 => Ok(Self::NYTCooking),
            1 => Ok(Self::AllRecipes),
            _ => Err(anyhow!("invalid publisher value")),
        }
    }
}

use diesel::backend::Backend;
use diesel::serialize;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Integer;
impl<DB> ToSql<Integer, DB> for Publisher
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: std::io::Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        (*self as i32).to_sql(out)
    }
}

#[derive(Debug)]
enum IngredientsList {
    /// Vec of component IDs and associated ingredient IDs with quantity
    HasSubComponents(Vec<(usize, Vec<usize>)>),
    /// Vec of ingredient IDs with quantity
    NoSubComponents(Vec<usize>),
}

/// Table to look up any ingredient, component, or tag by name or ID
struct Ingredients {
    /// Bijective map to look up ingredient names by ID or IDs by name
    ingredients: BiMap<usize, String>,
    /// Bijective map to look up component names by ID or IDs by name
    components: BiMap<usize, String>,
    /// Bijective map to look up tag names by ID or IDs by name
    tags: BiMap<usize, String>,
}

/// Table to look up a recipe by ID
struct Recipes {
    /// Lookup recipes by ID
    recipes: HashMap<usize, Recipe>,
}

pub fn pull_recipes(q_tag: Option<String>) -> Vec<(i32, String)> {
    use crate::schema::*;
    let connection: PgConnection = crate::establish_connection();

    match q_tag {
        Some(tag) => {
            recipes::table
                .inner_join(recipe_ingredient::table.inner_join(ingredients::table))
                .inner_join(recipe_tag::table.inner_join(tags::table))
                .filter(tags::name.eq(tag))
                .select((recipes::id, ingredients::name))
                .load::<(i32, String)>(&connection)
                .unwrap()
        }
        None => {
            recipes::table
                .inner_join(recipe_ingredient::table.inner_join(ingredients::table))
                .inner_join(recipe_tag::table.inner_join(tags::table))
                .select((recipes::id, ingredients::name))
                .load::<(i32, String)>(&connection)
                .unwrap()
        }
    }
}

use crate::diesel::prelude::*;
use crate::scrapers::nytcooking::Nytc;
use itertools::join;
use reqwest;
pub fn consolodate() {
    let connection: PgConnection = crate::establish_connection();

    let all: Vec<Nytc> = crate::process_ny::pull_recipes(vec![], 0, 0, None);

    for recipe in all {
        match parse_recipe(&recipe) {
            Ok(response) => {
                let new_recipe = NewRecipe {
                    title: recipe.title.clone(),
                    source: Publisher::NYTCooking,
                    url: "https://cooking.nytimes.com/recipes/".to_string() + &recipe.url_id,
                    yields: recipe.yield_.clone(),
                    time: None,
                    description: recipe.description.clone(),
                    steps: recipe.steps.clone(),
                    num_ratings: recipe.ratings,
                    avg_rating: recipe.rating.to_f32(),
                };

                let inserted: QueryableRecipe = diesel::insert_into(recipes::table)
                    .values(&new_recipe)
                    .get_results(&connection)
                    .unwrap()
                    .pop()
                    .unwrap();
                let recipe_id = inserted.id;

                for parsed in response {
                    {
                        let ingredient: Option<Ingredient> = {
                            use crate::schema::ingredients::dsl::*;
                            ingredients
                                .filter(name.eq(&parsed.TAG_NAME))
                                .first(&connection)
                                .optional()
                                .unwrap()
                        };

                        match ingredient {
                            Some(ingredient) => {
                                let new_recipe_ingredient = NewRecipeIngredient {
                                    recipes_id: recipe_id,
                                    ingredients_id: ingredient.id,
                                    sub_components_id: None,
                                    quantity: parsed.SERVING_WEIGHT_GRAMS,
                                    quantity_note: None,
                                };
                                diesel::insert_into(recipe_ingredient::table)
                                    .values(&new_recipe_ingredient)
                                    .execute(&connection)
                                    .unwrap();
                                }
                            None => {
                                let new_ingredient = NewIngredient {
                                    name: parsed.TAG_NAME,
                                };
                                let inserted: Ingredient = diesel::insert_into(ingredients::table)
                                    .values(&new_ingredient)
                                    .get_results(&connection)
                                    .unwrap()
                                    .pop()
                                    .unwrap();
                                let ingredient_id = inserted.id;

                                let new_recipe_ingredient = NewRecipeIngredient {
                                    recipes_id: recipe_id,
                                    ingredients_id: ingredient_id,
                                    sub_components_id: None,
                                    quantity: parsed.SERVING_WEIGHT_GRAMS,
                                    quantity_note: None,
                                };
                                diesel::insert_into(recipe_ingredient::table)
                                    .values(&new_recipe_ingredient)
                                    .execute(&connection)
                                    .unwrap();
                            }
                        }
                    }
                }

                for new_tag in recipe.tags {
                    {
                        let tag: Option<Tag> = {
                            use crate::schema::tags::dsl::*;
                            tags.filter(name.eq(&new_tag))
                                .first(&connection)
                                .optional()
                                .unwrap()
                        };

                        match tag {
                            Some(tag) => {
                                let new_recipe_tag = NewRecipeTag {
                                    recipes_id: recipe_id,
                                    tags_id: tag.id,
                                };
                                diesel::insert_into(recipe_tag::table)
                                    .values(&new_recipe_tag)
                                    .execute(&connection)
                                    .unwrap();
                            }
                            None => {
                                let new_tag = NewTag { name: new_tag };
                                let inserted: Tag = diesel::insert_into(tags::table)
                                    .values(&new_tag)
                                    .get_results(&connection)
                                    .unwrap()
                                    .pop()
                                    .unwrap();
                                let tag_id = inserted.id;

                                let new_recipe_tag = NewRecipeTag {
                                    recipes_id: recipe_id,
                                    tags_id: tag_id,
                                };
                                diesel::insert_into(recipe_tag::table)
                                    .values(&new_recipe_tag)
                                    .execute(&connection)
                                    .unwrap();
                            }
                        }
                    }
                }
            }

            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}

#[derive(Deserialize)]
struct Response {
    SERVING_WEIGHT_GRAMS: f32,
    TAG_NAME: String,
    TAG_ID: usize,
}

fn parse_recipe(recipe: &Nytc) -> Result<Vec<Response>> {
    let client = reqwest::blocking::Client::new();
    let mut query_string = join(
        recipe
            .ingredients
            .ingredients
            .iter()
            .map(|(qty, phrase)| qty.clone() + " " + &phrase),
        "\n",
    );

    query_string.retain(|c| c != ')' && c != '(');

    println!("{}", query_string);

    let mut map = std::collections::HashMap::new();
    map.insert(String::from("line_delimited"), String::from("true"));
    map.insert(String::from("include_subrecipe"), String::from("false"));
    map.insert(String::from("use_raw_foods"), String::from("true"));
    map.insert(String::from("use_branded_foods"), String::from("false"));
    map.insert(String::from("query"), query_string);

    Ok(client
        .post("https://trackapi.nutritionix.com/v2/natural/tags")
        .json(&map)
        .send()?
        .json::<Vec<Response>>()?)
}
