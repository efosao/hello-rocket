
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate reqwest;

use rocket_contrib::json::{JsonValue};
use rocket_contrib::templates::Template;
use reqwest::Error;
use serde::{Serialize, Deserialize};

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

#[derive(Serialize, Deserialize, Debug)]
struct PageData {
    desc: String,
    title: String,
}

#[get("/")]
fn index() -> Template {
    let context = PageData {
        title: "Hello World".to_string(),
        desc: "This is a simple page description".to_string(),
    };
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, todo])
        .launch();
}

#[get("/todo")]
fn todo() -> Result<JsonValue, Error> {
    return Ok(json!(get_data()?));
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
}

fn get_data() -> Result<Vec<User>, Error> {
    // https://api.github.com/repos/rust-lang-nursery/rust-cookbook/stargazers
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                                owner = "rust-lang-nursery",
                                repo = "rust-cookbook");
    println!("request_url: {}", request_url);
    let mut response = reqwest::get(&request_url)?;
    let users: Vec<User> = response.json()?;
    Ok(users)
}
