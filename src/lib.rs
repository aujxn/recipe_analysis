#[macro_use]
/// Simple ingredient co-occurrence relationship to investigate if structure exists in
/// the network of cooking ingredients.
pub mod co_occurrence;
/// Expanded relationship to allow ingredients to participate in multiple communities
pub mod expanded;
/// Hierarchy of partitions created through one of the modularity based graph partitioning
/// algorithms
pub mod hierarchy;
/// Wrapper module to run Blondel et. al.'s implementation of Louvain modulatiry partitioning
pub mod louvain;
pub mod recipe;

//pub mod graph_explorer;

use anyhow::Result;
use matrixlab::matrix::sparse::SparseMatrix;
use tokio_postgres::{Client, Config, NoTls};

#[derive(Clone, Copy)]
pub enum Databases {
    Recipes,
    RecipeAPI,
}

impl Databases {
    fn to_str(&self) -> &str {
        match self {
            Databases::Recipes => "recipes",
            Databases::RecipeAPI => "recipe_api",
        }
    }
}

pub async fn connect_db(db: Databases) -> Result<Client> {
    let (client, connection) = Config::new()
        .user("austen")
        .dbname(db.to_str())
        .host("localhost")
        .connect(NoTls)
        .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            panic!("connection error: {}", e);
        }
    });

    Ok(client)
}

// exports the matrix into the format that graph-embed binary can intake
pub fn export(matrix: &SparseMatrix<usize>) -> String {
    let rows = matrix.num_rows();
    let cols = matrix.num_columns();
    let indptr = matrix
        .get_rows()
        .iter()
        .map(|x| format!("\n{}", x))
        .collect::<String>();
    let indices = matrix
        .get_columns()
        .iter()
        .map(|x| format!("\n{}", x))
        .collect::<String>();
    let data = matrix
        .get_data()
        .iter()
        .map(|x| format!("\n{}", x))
        .collect::<String>();

    format!("{}\n{}{}{}{}", rows, cols, indptr, indices, data)
}
