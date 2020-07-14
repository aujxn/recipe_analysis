#[macro_use]
pub mod co_occurrence;
pub mod expanded;
pub mod hierarchy;
pub mod louvain;
pub mod recipe;

use anyhow::Result;
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
