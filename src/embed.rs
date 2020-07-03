use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

pub fn embed() {
    let result = Command::new("../graph-embed/build/examples/embedder")
        //.arg("/home/austen/Documents/school/research/recipe_analysis/temp/expanded_coolist")
        .arg("./temp/coolist")
        .output()
        .unwrap();

    println!("{}", &std::str::from_utf8(&result.stdout).unwrap());
}

pub fn plot(ingredients: &Vec<String>) {
    let ingredients = ingredients.iter().join("\n");
    let mut ingredients_file = File::create("temp/ing.temp").unwrap();
    ingredients_file.write_all(ingredients.as_bytes()).unwrap();

    /*
    let _ = Command::new("python")
        .args(&[
            "../graph-embed/scripts/plot-graph.py",
            "-graph",
            "temp/mat.temp",
            "-part",
            "temp/part.temp",
            "-coords",
            "temp/coords.temp",
            "-ingredients",
            "temp/ing.temp",
            "-o",
            "temp/plot.temp",
        ])
        .output()
        .unwrap();
    */
}
