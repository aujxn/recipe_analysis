extern crate recipe_analysis;
pub fn main() {
    let recipes = recipe_analysis::recipe::pull_recipes(Some("Dessert".to_string()));
    let (_recipe_count, coolist, ingredienst_vec) =
        recipe_analysis::co_occurrence::make_coolist(recipes, vec!["lemon zest".to_string()]);
    //recipe_analysis::embed::embed();
    //
    for x in coolist.iter().take(9) {
        println!(
            "{} {} {}",
            ingredienst_vec[x[0] as usize], ingredienst_vec[x[1] as usize], x[2]
        );
    }
}
