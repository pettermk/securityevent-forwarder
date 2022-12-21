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
    project: HashMap<String, Value>,
    newIssues: Vec<HashMap<String, Value>>,
    removedIssues: Vec<HashMap<String, Value>>
}

#[post("/store", data = "<input>")]
async fn store(input: Json<NewSnykEvent>) -> std::io::Result<()> {
    snyk-webhook-api::crud::create_post(input.into_inner(), &connection)
        .map(|post| post_created(post))
        .map_err(|error| error_status(error))
    println!("Test");
    println!("{:?}", input);
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, store])
}
