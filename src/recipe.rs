use crate::connect_db;
use crate::Databases;
use anyhow::{anyhow, Result};
use futures::{pin_mut, TryStreamExt};
use indexmap::IndexSet;
use itertools::Itertools;
use tokio_postgres::types::ToSql;

#[derive(Debug)]
pub struct Recipe {
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
    ingredients: Vec<String>,
    steps: Vec<String>,
    comments: Option<Vec<(String, usize)>>,
}

#[derive(Debug)]
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

pub async fn query_filtered_recipes(
    tag: Option<String>,
    all_ingredients: Option<Vec<String>>,
    one_ingredient: Option<Vec<String>>,
) -> Result<Vec<(i32, Vec<String>)>> {
    let mut one_ingredient_query = None;
    let mut all_ingredients_query = None;
    let mut tag_query = None;
    let mut variable_counter = 1;
    let mut params = vec![];
    let client = connect_db(Databases::Recipes).await?;

    if let Some(mut list) = one_ingredient {
        if list.len() != 0 {
            let mut query = String::from(
                "SELECT DISTINCT recipes.id
                FROM recipes 
                INNER JOIN recipe_ingredient
                ON recipes.id = recipe_ingredient.recipes_id
                INNER JOIN ingredients
                ON recipe_ingredient.ingredients_id = ingredients.id
                WHERE ",
            );

            query += &(0..list.len())
                .map(|x| format!("ingredients.name = ${}", variable_counter + x))
                .join(" OR ");

            variable_counter += list.len();
            one_ingredient_query = Some(query);
            params.append(&mut list);
        }
    }

    if let Some(mut list) = all_ingredients {
        if list.len() != 0 {
            all_ingredients_query = Some(
                (0..list.len())
                    .map(|x| {
                        format!(
                            "SELECT DISTINCT recipes.id
                             FROM recipes 
                             INNER JOIN recipe_ingredient
                             ON recipes.id = recipe_ingredient.recipes_id
                             INNER JOIN ingredients
                             ON recipe_ingredient.ingredients_id = ingredients.id
                             WHERE ingredients.name = ${}",
                            variable_counter + x
                        )
                    })
                    .join(" INTERSECT "),
            );
            variable_counter += list.len();
            params.append(&mut list);
        }
    }

    if let Some(tag) = tag {
        tag_query = Some(format!(
            "SELECT DISTINCT recipes.id
             FROM recipes 
             INNER JOIN recipe_tag
             ON recipes.id = recipe_tag.recipes_id
             INNER JOIN tags
             ON recipe_tag.tags_id = tags.id
             WHERE tags.name = ${}",
            variable_counter
        ));
        params.push(tag);
    }

    let sub_query = [one_ingredient_query, all_ingredients_query, tag_query]
        .iter()
        .filter_map(|x| x.as_ref())
        .join(" INTERSECT ");

    let query = format!(
        "SELECT recipes.id, array_agg(DISTINCT ingredients.name) as ingredients_list
                         FROM recipes
                         INNER JOIN recipe_ingredient
                         ON recipes.id = recipe_ingredient.recipes_id
                         INNER JOIN ingredients
                         ON recipe_ingredient.ingredients_id = ingredients.id
                         WHERE recipes.id IN ({})
                         GROUP BY recipes.id",
        sub_query
    );

    let params = params.iter().map(|p| p as &dyn ToSql);
    let query = client.prepare(&query).await?;
    let row_stream = client.query_raw(&query, params).await?;

    pin_mut!(row_stream);
    let mut data = vec![];

    while let Some(row) = row_stream.try_next().await? {
        data.push((row.get(0), row.get(1)));
    }

    Ok(data)
}
