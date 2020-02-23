extern crate recipe_analysis;

use indexmap::IndexSet;
use recipe_analysis::embed;
use recipe_analysis::nytc;
use recipe_analysis::nytc::Nytc;
use recipe_analysis::process_ny;
use recipe_analysis::scraper;
use recipe_analysis::table::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "tool for recipe analysis")]
enum Opt {
    Scrape {
        /// Argument must be nytc or allrecipes
        #[structopt(short, long)]
        website: String,
    },
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
                nytc::crawl();
            } else if website == "all_recipes" {
                scraper::crawl();
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

            let (_ingredients_map, ingredients_vec, _ingredients_count, ingredient_cooccurrence) =
                ingredient_map(&parsed_ingredients);

            ingredient_cooccurrence.make_coolist();

            embed::embed();
            if plot {
                embed::plot(&ingredients_vec);
            }
        }
    }
}
