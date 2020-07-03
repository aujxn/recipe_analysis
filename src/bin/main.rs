extern crate recipe_analysis;

use indexmap::IndexSet;
use recipe_analysis::{
    embed, expanded, hierarchy::Hierarchy, louvain, process_ny, scrapers::allRecipes,
    scrapers::nytcooking, scrapers::nytcooking::Nytc, table,
};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "tool for recipe analysis")]
enum Opt {
    Scrape {
        /// Argument must be nytc or allrecipes
        #[structopt(short, long)]
        website: String,
    },
    /* TODO
    Dump {

    },
    */
    AnalyzeNYT {
        /// Filter by tags
        #[structopt(short, long)]
        tags: Vec<String>,

        /// Filter by ingredients
        #[structopt(short, long)]
        ingredients: Vec<String>,

        /// Filter by minimum vote count
        #[structopt(short, long, default_value = "0")]
        votes: i32,

        /// Filter by minimum rating (0 to 5)
        #[structopt(short, long, default_value = "0")]
        rating: i32,

        /// Filter by author
        #[structopt(short, long)]
        author: Option<String>,

        /// Title must contain the substring
        #[structopt(long)]
        title: Option<String>,

        /// Plots the clustering
        #[structopt(short, long)]
        plot: bool,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Scrape { website } => {
            if website == "nytc" {
                nytcooking::crawl();
            } else if website == "all_recipes" {
                allRecipes::crawl();
            } else {
                println!("Valid websites to scrape include:\n\n nytc \n\n all_recipes");
            }
        }
        Opt::AnalyzeNYT {
            tags,
            ingredients,
            votes,
            rating,
            author,
            title,
            plot,
        } => {
            let mut recipes = process_ny::pull_recipes(tags, votes, rating, author);

            if let Some(title) = title {
                recipes = process_ny::filter_title(recipes, &title)
            }

            // Vec of recipes --- each recipe is an IndexSet of ingredients as Strings
            let parsed_ingredients = process_ny::parse_ingredients(&recipes);

            let mut data: Vec<(Nytc, IndexSet<String>)> = recipes
                .into_iter()
                .zip(parsed_ingredients.into_iter())
                .collect();

            if ingredients.len() > 0 {
                let filter: IndexSet<String> = ingredients.into_iter().collect();
                data = data
                    .into_iter()
                    .filter(|(_, ingredients)| filter.is_subset(ingredients))
                    .collect();
            }

            let (_recipes, parsed_ingredients): (Vec<Nytc>, Vec<IndexSet<String>>) =
                data.into_iter().unzip();

            let (
                ingredients_map,
                ingredients_vec,
                _ingredients_count,
                ingredient_cooccurrence,
                recipe_ingredient,
            ) = table::ingredient_map(&parsed_ingredients);

            ingredient_cooccurrence.make_coolist();
            embed::embed();
            embed::plot(&ingredients_vec);

            /*
                let mut expanded_ingredient_relation =
                    expanded::ExpandedIngredientRelation::new(recipe_ingredient, ingredients_vec.len());

                let choices = ["chicken", "soy sauce", "brown sugar"];
                let choices_id = choices
                    .iter()
                    .map(|x| *ingredients_map.get(&String::from(*x)).unwrap())
                    .collect();
                expanded_ingredient_relation.connect_clique(&choices_id);

                expanded_ingredient_relation.build_coolist();

                let n = expanded_ingredient_relation.number_of_vertices();

                let interpolation_matrices = louvain::louvain(n);
                let hierarchy = Hierarchy::new(
                    interpolation_matrices,
                    ingredients_vec,
                    ingredients_map.clone(),
                    expanded_ingredient_relation,
                );

                let communities = hierarchy.generate_recipes(2).unwrap();

                for comm in communities {
                    if choices
                        .iter()
                        .by_ref()
                        .all(|x| comm.get(&String::from(*x)).is_some())
                    {
                        println!();
                        let mut ingredients = comm.iter().collect::<Vec<_>>();
                        ingredients.sort_unstable_by(|a, b| b.1.len().cmp(&a.1.len()));
                        let is_original = ingredients
                            .iter()
                            .take(8)
                            .fold(IndexSet::new(), |acc, (_, recipes)| {
                                acc.intersection(recipes).cloned().collect()
                            });
                        println!("shared: {:?}", is_original);
                        for (ingredient, recipes) in ingredients.iter() {
                            print!("{} - {} ", recipes.len(), ingredient);
                            /*
                            for recipe in recipes.iter() {
                                print!("{} ", recipe);
                            }
                            */
                            println!();
                        }
                    }
                }

                //embed::embed();
                /*
                if plot {
                    embed::plot(&ingredients_vec);
                }
                */
            */
        }
    }
}
