use super::schema::{nyt, recipes_table};

#[derive(Queryable, Debug, Clone)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub time: f32,
    pub yields: i32,
    pub ingredients: String,
    pub instructions: String,
    pub rating: f32,
    pub reviews: i32,
    pub url_id: i32,
}

#[derive(Insertable)]
#[table_name = "recipes_table"]
pub struct NewRecipe {
    pub title: String,
    pub time: f32,
    pub yields: i32,
    pub ingredients: String,
    pub instructions: String,
    pub rating: f32,
    pub reviews: i32,
    pub url_id: i32,
}

#[derive(Queryable, Debug, Clone)]
pub struct NytcQueryable {
    pub id: i32,
    pub title: String,
    pub author: Option<String>,
    pub yield_: String,
    pub time: Option<String>,
    pub description: Option<String>,
    pub featured: Option<String>,
    pub tags: Vec<String>,
    pub ratings: i32,
    pub rating: i32,
    pub sub_components: Option<Vec<String>>,
    pub indices: Option<Vec<i32>>,
    pub ingredients: Vec<String>,
    pub quantities: Vec<String>,
    pub steps: Vec<String>,
    pub comments: Option<Vec<String>>,
    pub comment_votes: Option<Vec<i32>>,
    pub url_id: String,
}

#[derive(Insertable)]
#[table_name = "nyt"]
pub struct NytcInsertable {
    pub title: String,
    pub author: Option<String>,
    pub yield_: String,
    pub time: Option<String>,
    pub description: Option<String>,
    pub featured: Option<String>,
    pub tags: Vec<String>,
    pub ratings: i32,
    pub rating: i32,
    pub sub_components: Option<Vec<String>>,
    pub indices: Option<Vec<i32>>,
    pub ingredients: Vec<String>,
    pub quantities: Vec<String>,
    pub steps: Vec<String>,
    pub comments: Option<Vec<String>>,
    pub comment_votes: Option<Vec<i32>>,
    pub url_id: String,
}
