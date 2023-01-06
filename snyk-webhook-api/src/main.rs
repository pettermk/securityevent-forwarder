mod models;
mod schema;
mod crud;

#[macro_use] extern crate rocket;

use rocket::data::{Data};
use rocket::request::{self, Request, FromRequest, Outcome};
use rocket::tokio::io::AsyncReadExt;
use rocket::{serde::json::Json};
use models::NewSnykEvent;
use models::from_new_snyk_event_dto;
use models::NewSnykEventDto;
use crud::create_snyk_event;

struct Headers(String);

#[derive(Debug)]
enum ApiTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Headers {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        for h in request.headers().iter() {
            println!("{}: {}", h.name(), h.value());
        }
        println!("{}", request.to_string());
        let token = request.headers().get_one("token");
        match token {
            Some(token) => {
                // check validity
                Outcome::Success(Headers(token.to_string()))
            }
            None => Outcome::Success(Headers("".to_string()))
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/justlogit", data = "<input>")]
async fn justlogit(key: Headers, input: Data<'_>) -> std::io::Result<()> {
    let mut d = input.open("256 kB".parse().unwrap());
    let mut content = String::new();
    d.read_to_string(&mut content).await;
    println!("{}", content);
    Ok(())
}

#[post("/store", data = "<input>")]
async fn store(key: Headers, input: Json<NewSnykEventDto>) -> std::io::Result<()> {
    let new_snyk_event: NewSnykEvent = from_new_snyk_event_dto(input.into_inner());
    let mut conn = snyk_webhook_api::establish_connection();
    create_snyk_event(new_snyk_event, &mut conn)
         .map(|snyk_event| Json(snyk_event))
         .expect("msg");
    Ok(())
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, store, justlogit])
}
