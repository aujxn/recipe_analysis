# Culinary Computation: Data Analysis for Novel Recipe Generation

## Project Condition
Currently this project is in its early stages. The end goal is to provide
tools to procedurally generate recipes or changes to recipes using statistical
models and machine learning. So far working components include:

- Scrapers to collect recipe data from the websites New York Times Cooking and AllRecipes
- Command line tool to select recipes from the database given some predicates
- Tools to build some data structures from the selected data like ingredient co-occurrence graphs and ingredient vector representations of recipes
- Integration with tools that embed the ingredients into 3 dimensional space by applying a restrictive algebraic multigrid method known as the modularity functional to the co-occurrence graph

## Usage
Unfortunately the project isn't well documented yet and would be difficult to clone and experiment with yourself. There are absolute file paths that are hard coded into the Rust code and you would need my database to access any of the data.

If you are interested in trying to run what I have so far, it should still be possible. To generate your own database you could run the scrapers, it takes about a day to scrape NYTC. If you would like a pg_dump of my postgres I can also provide that on request.

Dependencies not included in this repository include, rustup, Cargo, Docker and [my fork of mtlynch's ingredient-phrase-tagger](https://github.com/aujxn/ingredient-phrase-tagger). The training file for the conditional random field is included in that repository so it is ready to run. The accuracy isn't great
but it is good enough to get some interesting clustering results.

## To build the project:
Clone the repo and initialize the submodules
```
git clone git@github.com:aujxn/research
git clone git@github.com:aujxn/ingredient-phrase-tagger
cd research
git submodule init
git submodule update
```

Build linalgcpp
```
cd linalgcpp
mkdir build && cd build && cmake .. && make
cd ..
```

Build graph-embed
```
cd graph-embed
mkdir build && cd build && cmake .. && make
cd ..
```

Create a python virtual environment in the project root and get the dependencies for graph-embed
```
python3 -m venv pyvenv
source pyvenv/bin/activate
pip install --upgrade pip
pip install -r graph-embed/requirements.txt
```

Build the recipe_analysis tools 
```
cd recipe_analysis
cargo build --release
```

To run the program from the recipe_analysis module
```
cargo run --bin main -- --help
```
will provide what command line options are available. In the future I will remove the
absolute path names and change the database connections to environment variables.
