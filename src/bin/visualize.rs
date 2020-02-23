use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut ingredients_file = File::open("cocktail_ingredients").unwrap();

    let mut ingredients = String::new();
    ingredients_file.read_to_string(&mut ingredients).unwrap();

    let ingredients: Vec<String> = ingredients.split("\n").map(|x| x.to_string()).collect();

    let mut part_file = File::open(
        "/home/austen/Documents/school/research/graph-embed/build/examples/temp/part.temp",
    )
    .unwrap();
    let mut part = String::new();

    part_file.read_to_string(&mut part).unwrap();

    let mut lines = part.split("\n");
    let mut line = lines.next().unwrap().split(" ");

    let nodes: usize = line.next().unwrap().parse().unwrap();
    let level_count: usize = line.next().unwrap().parse().unwrap();

    line = lines.next().unwrap().split(" ");

    let nodes_per_level: Vec<usize> = line
        .take(level_count)
        .map(|count| count.parse().unwrap())
        .collect();

    let hierarchies: Vec<Vec<Vec<usize>>> = nodes_per_level
        .iter()
        .cloned()
        .map(|nodes| {
            lines
                .by_ref()
                .take(nodes)
                .map(|vals| {
                    vals.split(" ")
                        .filter(|x| x != &"" && x != &"\n" && x != &" ")
                        .map(|val| val.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    for agg in hierarchies[0].iter() {
        for node in agg {
            print!("{}, ", ingredients[*node]);
        }
        println!("\n\n");
    }
}
