mod models;
mod schema;
mod crud;

#[macro_use] extern crate rocket;
use rocket::{serde::json::Json};
use models::NewSnykEvent;
use crud::create_snyk_event;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/store", data = "<input>")]
async fn store(input: Json<NewSnykEvent>) -> std::io::Result<()> {
    let mut conn = snyk_webhook_api::establish_connection();
    create_snyk_event(input.into_inner(), &mut conn)
         .map(|snyk_event| Json(snyk_event))
         .expect("msg");
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, store])
}
