#[macro_use] extern crate rocket;
use rocket::{serde::{Deserialize, json::Json, json::Value}, Data};
use std::collections::HashMap;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SnykEvent {
    project_id: String,
    test: Vec<HashMap<String, Value>>
}

#[post("/store", data = "<input>")]
async fn store(input: Json<SnykEvent>) -> std::io::Result<()> {
    println!("Test");
    println!("{:?}", input);
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, store])
}
