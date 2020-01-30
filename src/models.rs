use super::schema::recipes_table;

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
