extern crate select;
use crate::error::Error;
use crate::*;
use rand::Rng;
use regex::Regex;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

extern crate diesel;

use crate::diesel::prelude::*;
use crate::models::{NytcInsertable, NytcQueryable};
use indexmap::set::IndexSet;
use reqwest::blocking::Client;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::de::from_str;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredients {
    // Name, starting index
    pub sub_components: Option<Vec<(String, i32)>>,

    // Quantity, ingredient - quantity is a string over a
    // float because of NYTC use of fraction characters
    // quantity might also be an empty string if none was listed
    pub ingredients: Vec<(String, String)>,
}

impl Ingredients {
    pub fn display(&self) {
        println!("Ingredients:");
        for ing in self.ingredients.iter() {
            if ing.0 != "" {
                print!("{} ", ing.0);
            }
            println!("{}", ing.1);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Rating {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Rating {
    pub fn new(rating: i32) -> Self {
        match rating {
            1 => Rating::One,
            2 => Rating::Two,
            3 => Rating::Three,
            4 => Rating::Four,
            5 => Rating::Five,
            _ => Rating::Zero,
        }
    }
}

#[derive(Debug)]
pub struct Nytc {
    pub title: String,
    pub author: Option<String>,
    pub yield_: String,
    pub time: Option<String>,
    pub description: Option<String>,
    pub featured: Option<String>,
    pub tags: Vec<String>,
    pub ratings: i32,
    pub rating: Rating,
    pub ingredients: Ingredients,
    pub steps: Vec<String>,
    pub comments: Option<Vec<(String, i32)>>,
    pub url_id: String,
}

impl Nytc {
    pub fn display(&self) {
        println!(
            "\n------------------------------------------\n\
             {}\n------------------------------------------",
            self.title
        );
        if let Some(author) = &self.author {
            println!("author: {}", author);
        }
        println!("yields: {}", self.yield_);
        if let Some(time) = &self.time {
            println!("time: {}", time);
        }
        if let Some(description) = &self.description {
            println!("\n{}\n", description);
        }
        if let Some(featured) = &self.featured {
            println!("{}\n", featured);
        }
        print!("tags: ");
        for tag in &self.tags {
            print!("{} ", tag);
        }
        println!("\n{:?} stars - {} ratings\n", self.rating, self.ratings);
        self.ingredients.display();
        println!("\nURL: {}", self.url_id);
    }

    pub fn serialize(self) -> NytcInsertable {
        let (sub_components, indices) = match self.ingredients.sub_components {
            None => (None, None),
            Some(components) => {
                let (a, b) = components.into_iter().unzip();
                (Some(a), Some(b))
            }
        };

        let (quantities, ingredients) = self.ingredients.ingredients.into_iter().unzip();

        let (comments, votes) = match self.comments {
            None => (None, None),
            Some(comments) => {
                let (a, b) = comments.into_iter().unzip();
                (Some(a), Some(b))
            }
        };

        NytcInsertable {
            title: self.title,
            author: self.author,
            yield_: self.yield_,
            time: self.time,
            description: self.description,
            featured: self.featured,
            tags: self.tags,
            ratings: self.ratings,
            rating: self.rating as i32,
            sub_components: sub_components,
            indices: indices,
            ingredients: ingredients,
            quantities: quantities,
            steps: self.steps,
            comments: comments,
            comment_votes: votes,
            url_id: self.url_id,
        }
    }

    pub fn new(recipe: NytcQueryable) -> Self {
        let sub_components;
        if recipe.sub_components.is_some() && recipe.indices.is_some() {
            sub_components = Some(
                recipe
                    .sub_components
                    .unwrap()
                    .into_iter()
                    .zip(recipe.indices.unwrap().into_iter())
                    .collect(),
            );
        } else {
            sub_components = None;
        }

        let ingredients = Ingredients {
            ingredients: recipe
                .quantities
                .into_iter()
                .zip(recipe.ingredients.into_iter())
                .collect(),
            sub_components,
        };

        let comments;
        if recipe.comments.is_some() && recipe.comment_votes.is_some() {
            comments = Some(
                recipe
                    .comments
                    .unwrap()
                    .into_iter()
                    .zip(recipe.comment_votes.unwrap().into_iter())
                    .collect(),
            );
        } else {
            comments = None;
        }

        Self {
            title: recipe.title,
            author: recipe.author,
            yield_: recipe.yield_,
            time: recipe.time,
            description: recipe.description,
            featured: recipe.featured,
            tags: recipe.tags,
            ratings: recipe.ratings,
            rating: Rating::new(recipe.rating),
            ingredients: ingredients,
            steps: recipe.steps,
            comments: comments,
            url_id: recipe.url_id,
        }
    }
}

pub fn crawl() {
    use crate::schema::nyt::dsl::*;
    let connection: PgConnection = establish_connection();
    let mut rng = rand::thread_rng();

    let user = "";
    let psw = "";

    let scraped: IndexSet<String> = nyt
        .load::<NytcQueryable>(&connection)
        .expect("failed to query")
        .iter()
        .map(|recipe| recipe.url_id.clone())
        .collect();

    // to_scrape, scraped
    let index_sets: Arc<Mutex<(IndexSet<String>, IndexSet<String>)>> =
        Arc::new(Mutex::new((IndexSet::new(), scraped)));

    let index_sets2 = Arc::clone(&index_sets);
    let _link_finder = thread::spawn(move || {
        let search = "https://cooking.nytimes.com/search?q=&page=";
        let client = Client::new();

        for i in 1..421 {
            let mut rng = rand::thread_rng();
            let url = Url::parse((String::from(search) + &i.to_string()).as_str()).unwrap();
            let response = match client.get(url).basic_auth(user, Some(psw)).send() {
                Ok(response) => response,
                Err(_) => continue,
            };

            let html_text = match response.text() {
                Ok(text) => text,
                Err(_) => continue,
            };

            // unwrap is fine here because this is a valid regex expression
            let link_regex = Regex::new(r"/recipes/(?P<url_id>\d+)").unwrap();
            let links: IndexSet<String> = link_regex
                .captures_iter(&html_text)
                .map(|cap| String::from(&cap["url_id"]))
                .collect();

            {
                let mut index_sets = index_sets2.lock().unwrap();
                let new_links: Vec<String> = links.difference(&index_sets.1).cloned().collect();
                for new in new_links {
                    index_sets.0.insert(new);
                }
            }

            let sleep = Duration::from_secs(rng.gen_range(1, 6));
            thread::sleep(sleep);
        }
    });

    let client = Client::new();
    let mut failed: Vec<String> = vec![];
    loop {
        let recipe_id;
        {
            let mut index_sets = index_sets.lock().unwrap();
            recipe_id = index_sets.0.pop();
            if recipe_id.is_some() {
                index_sets.1.insert(recipe_id.clone().unwrap());
            };
        }

        let recipe;
        if let Some(url) = recipe_id {
            if let Ok(result) = scrape(&client, &url, user, psw) {
                recipe = result;
                let _: NytcQueryable = diesel::insert_into(schema::nyt::table)
                    .values(&recipe.serialize())
                    .get_result(&connection)
                    .expect("error inserting new nytc");
            } else {
                failed.push(url);
            }
        };

        let sleep = Duration::from_secs(rng.gen_range(1, 6));
        println!("failed: {:?}", failed);
        thread::sleep(sleep);
    }
}

pub fn scrape(client: &Client, id: &str, user: &str, password: &str) -> Result<Nytc, Error> {
    // build the URL of the recipe to scrape
    let base = "https://cooking.nytimes.com/recipes/";
    let url = Url::parse((String::from(base) + id).as_str())?;

    // gets the http response for the recipe page
    let response = client.get(url).basic_auth(user, Some(password)).send()?;

    // build a document from the response to parse
    let html_text = response.text()?;
    let document = Document::from(html_text.as_str());

    let comments = get_comments(client, id, user, password).ok();
    let title = parse_title(&document)?;
    let author = parse_author(&document).ok();
    let yield_ = parse_yield(&document)?;
    let time = parse_time(&document).ok();
    let description = parse_description(&document).ok();
    let featured = parse_featured(&document).ok();
    let tags = parse_tags(&document)?;
    let rating = parse_rating(&document).unwrap_or(Rating::Zero);
    let ratings = parse_ratings(&document).unwrap_or(0);
    let ingredients = parse_ingredients(&document)?;
    let steps = parse_steps(&document)?;
    let url_id = String::from(id);

    println!(
        "title:         {}   \n\
         author:        {:?} \n\
         yield:         {}   \n\
         time:          {:?} \n\
         description:   {:?} \n\
         featured:      {:?} \n\
         tags:          {:?} \n\
         ratings:       {}   \n\
         rating:        {:?} \n\
         ingredients:   {:?} \n\
         steps:         {:?} \n\
         comments:      {:?} \n\
         url_id:        {}",
        title,
        author,
        yield_,
        time,
        description,
        featured,
        tags,
        ratings,
        rating,
        ingredients,
        steps,
        comments,
        url_id
    );

    Ok(Nytc {
        title,
        author,
        yield_,
        time,
        description,
        featured,
        tags,
        ratings,
        rating,
        ingredients,
        steps,
        comments,
        url_id,
    })
}

fn get_comments(
    client: &Client,
    id: &str,
    user: &str,
    password: &str,
) -> Result<Vec<(String, i32)>, Error> {
    let base = "https://cooking.nytimes.com/recipes/";

    // build an API request header to get the top 25 comments
    let api = "https://www.nytimes.com/svc/community/V3/requestHandler?";
    let callback = "callback=HelpfulNotes&";
    let method = "method=get&";
    let cmd = "cmd=GetCommentsReadersPicks&";
    let offset = "offset=0&sort=newest&url=";
    let header = String::from(api) + callback + method + cmd + offset + base + id;
    let comments_url = Url::parse(header.as_str())?;

    // raw json from comments API request
    let comments_response = client
        .get(comments_url)
        .basic_auth(user, Some(password))
        .send()?;

    // trim the json to have just the comments array
    let comments_json = comments_response.text()?;
    let start_chop = comments_json.find('[').ok_or(Error::ScrapeError)?;
    let end_chop = comments_json
        .find(",\"depthLimit\"")
        .ok_or(Error::ScrapeError)?;
    let comments_json_trim = &String::from(comments_json)[start_chop..end_chop];

    // deserialize the json into an array of comments with the vote counts
    let comments: Vec<(String, i32)> = from_str::<Vec<Value>>(comments_json_trim)?
        .iter()
        .filter_map(|obj| {
            if let Value::Object(values) = obj {
                let body = values.get("commentBody")?;
                let votes = values.get("recommendations")?;
                Some((String::from(body.as_str()?), votes.as_i64()? as i32))
            } else {
                None
            }
        })
        .collect();

    if comments.len() > 0 {
        Ok(comments)
    } else {
        Err(Error::ScrapeError)
    }
}

fn parse_title(document: &Document) -> Result<String, Error> {
    let title = document
        .find(Class("title-container").descendant(Attr("data-name", ())))
        .into_selection()
        .first()
        .ok_or(Error::ScrapeError)?
        .attr("data-name")
        .ok_or(Error::ScrapeError)?
        .parse()?;

    Ok(title)
}

fn parse_author(document: &Document) -> Result<String, Error> {
    let author = document
        .find(Name("a").and(Attr("data-author", ())))
        .into_selection()
        .first()
        .ok_or(Error::ScrapeError)?
        .attr("data-author")
        .ok_or(Error::ScrapeError)?
        .parse()?;

    Ok(author)
}

fn parse_yield(document: &Document) -> Result<String, Error> {
    let yield_ = String::from(
        document
            .find(Name("span").and(Attr("itemprop", "recipeYield")))
            .into_selection()
            .children()
            .first()
            .ok_or(Error::ScrapeError)?
            .as_text()
            .ok_or(Error::ScrapeError)?,
    );

    Ok(yield_)
}

fn parse_time(document: &Document) -> Result<String, Error> {
    let time = String::from(
        document
            .find(Name("span").and(Class("recipe-yield-value")))
            .filter(|node| !node.attr("itemprop").is_some())
            .next()
            .ok_or(Error::ScrapeError)?
            .first_child()
            .ok_or(Error::ScrapeError)?
            .as_text()
            .ok_or(Error::ScrapeError)?,
    );

    Ok(time)
}

fn parse_description(document: &Document) -> Result<String, Error> {
    let description = String::from(
        document
            .find(Class("topnote").and(Attr("itemprop", "description")))
            .into_selection()
            .children()
            .children()
            .first()
            .ok_or(Error::ScrapeError)?
            .as_text()
            .ok_or(Error::ScrapeError)?,
    );

    Ok(description)
}

fn parse_featured(document: &Document) -> Result<String, Error> {
    let featured = document
        .find(Class("related-article"))
        .into_selection()
        .first()
        .ok_or(Error::ScrapeError)?
        .children()
        .filter_map(|node| node.attr("href"))
        .next()
        .ok_or(Error::ScrapeError)?
        .parse()?;

    Ok(featured)
}

fn parse_tags(document: &Document) -> Result<Vec<String>, Error> {
    let mut tags = vec![];
    for tag in document
        .find(Class("tags-nutrition-container"))
        .into_selection()
        .children()
        .filter(Class("tag"))
        .iter()
    {
        tags.push(String::from(
            tag.first_child()
                .ok_or(Error::ScrapeError)?
                .as_text()
                .ok_or(Error::ScrapeError)?,
        ))
    }

    Ok(tags)
}

fn parse_rating(document: &Document) -> Result<Rating, Error> {
    let rating: i32 = document
        .find(Attr("itemprop", "ratingValue"))
        .into_selection()
        .children()
        .first()
        .ok_or(Error::ScrapeError)?
        .as_text()
        .ok_or(Error::ScrapeError)?
        .parse()?;

    Ok(Rating::new(rating))
}

fn parse_ratings(document: &Document) -> Result<i32, Error> {
    let ratings = document
        .find(Attr("itemprop", "ratingCount"))
        .into_selection()
        .children()
        .first()
        .ok_or(Error::ScrapeError)?
        .as_text()
        .ok_or(Error::ScrapeError)?
        .parse()?;

    Ok(ratings)
}

fn parse_ingredients(document: &Document) -> Result<Ingredients, Error> {
    let ingredients_nodes: Vec<Node> = document
        .find(Class("recipe-ingredients-wrap"))
        .into_selection()
        .first()
        .ok_or(Error::ScrapeError)?
        .descendants()
        .collect();

    let mut ingredients_counter = 0;
    let mut components = vec![];
    let mut indices = vec![];
    let mut ingredients = vec![];
    let mut quantities = vec![];

    for (node, next_node) in ingredients_nodes
        .iter()
        .zip(ingredients_nodes.iter().skip(1))
    {
        if node.is(Class("part-name")) {
            components.push(String::from(
                next_node.as_text().ok_or(Error::ScrapeError)?.trim(),
            ));
            indices.push(ingredients_counter);
        } else if node.is(Class("quantity")) {
            let qty = String::from(next_node.as_text().ok_or(Error::ScrapeError)?.trim());
            quantities.push(qty);
        } else if node.is(Class("ingredient-name")) {
            ingredients.push(String::from(
                next_node.as_text().ok_or(Error::ScrapeError)?.trim(),
            ));
            ingredients_counter += 1;
        }
    }

    let ingredients = quantities
        .into_iter()
        .zip(ingredients.into_iter())
        .collect();

    let sub_components;
    if components.len() > 0 {
        sub_components = Some(components.into_iter().zip(indices.into_iter()).collect());
    } else {
        sub_components = None;
    }

    let ingredients = Ingredients {
        sub_components,
        ingredients,
    };

    Ok(ingredients)
}

fn parse_steps(document: &Document) -> Result<Vec<String>, Error> {
    let steps = document
        .find(Class("recipe-steps"))
        .into_selection()
        .first()
        .ok_or(Error::ScrapeError)?
        .descendants()
        .filter_map(|node| node.as_text())
        .filter_map(|text| {
            let trim = text.trim();
            if trim == "" {
                None
            } else {
                Some(String::from(trim))
            }
        })
        .collect();

    Ok(steps)
}
