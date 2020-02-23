use std::process::Command;

pub fn embed() {
    let result = Command::new("../graph-embed/build/examples/embedder")
        .arg("/home/austen/Documents/school/research/recipe_analysis/temp/coolist")
        .output()
        .unwrap();

    println!("{}", &std::str::from_utf8(&result.stdout).unwrap());
}

pub fn plot() {
    let _ = Command::new("../pyvenv/bin/python3")
        .args(&[
            "../graph-embed/scripts/plot-graph.py",
            "-graph",
            "temp/mat.temp",
            "-part",
            "temp/part.temp",
            "-coords",
            "temp/coords.temp",
            "-o",
            "temp/plot.temp",
        ])
        .output()
        .unwrap();
}
