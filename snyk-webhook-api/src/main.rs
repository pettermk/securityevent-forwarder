mod models;
mod schema;
mod crud;

#[macro_use] extern crate rocket;
use rocket::{serde::json::Json};
use models::NewSnykEvent;
use models::from_new_snyk_event_dto;
use models::NewSnykEventDto;
use crud::create_snyk_event;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/store", data = "<input>")]
async fn store(input: Json<NewSnykEventDto>) -> std::io::Result<()> {
    let new_snyk_event: NewSnykEvent = from_new_snyk_event_dto(input.into_inner());
    let mut conn = snyk_webhook_api::establish_connection();
    create_snyk_event(new_snyk_event, &mut conn)
         .map(|snyk_event| Json(snyk_event))
         .expect("msg");
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, store])
}
