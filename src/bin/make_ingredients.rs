extern crate recipe_analysis;
pub fn main() {
    let recipes = recipe_analysis::recipe::pull_recipes(Some("Salads And Dressings".to_string()));
    let (coolist, ingredienst_vec) = recipe_analysis::co_occurrence::make_coolist(recipes, vec![]);
    //recipe_analysis::embed::embed();
}
