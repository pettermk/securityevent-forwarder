mod models;
mod schema;
mod lib;
mod crud;

#[macro_use] extern crate rocket;
use rocket::{serde::{Deserialize, json::Json, json::Value}, Data};
use std::collections::HashMap;
use models::NewSnykEvent;
use lib::establish_connection;
use crud::create_snyk_event;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// #[derive(Debug, PartialEq, Eq, Deserialize)]
// #[serde(crate = "rocket::serde")]
// struct SnykEvent {
//     project: HashMap<String, Value>,
//     newIssues: Vec<HashMap<String, Value>>,
//     removedIssues: Vec<HashMap<String, Value>>
// }

#[post("/store", data = "<input>")]
async fn store(input: Json<NewSnykEvent>) -> std::io::Result<()> {
    let mut conn = establish_connection();
    create_snyk_event(input.into_inner(), &mut conn);
    //     .map(|post| post_created(post))
    //     .map_err(|error| error_status(error));
    println!("Test");
    // println!("{:?}", input);
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, store])
}
