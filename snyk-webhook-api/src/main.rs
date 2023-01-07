mod models;
mod schema;
mod crud;

#[macro_use] extern crate rocket;

use crud::create_snyk_event;
use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use models::NewSnykEvent;
use models::NewSnykEventDto;
use models::from_new_snyk_event_dto;
use rocket::data::{Data, FromData, self};
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{status, content};
use rocket::serde::json::Json;
use rocket::serde::json::serde_json;
use rocket::tokio::io::AsyncReadExt;
use sha2::Sha256;
use std::env;
use std::path::PathBuf;
use hex::FromHex;

struct Payload {
    valid: bool,
    event: NewSnykEventDto,
}

#[derive(Debug)]
enum SignatureError {
    Missing,
}

type HmacSha256 = Hmac<Sha256>;

fn verify_hmac(payload: &[u8], signature: String) -> bool {
    match dotenv() {
        Ok(result) => result,
        Err(_) => PathBuf::new(),
    };
    let secret: String = env::var("SNYK_WEBHOOK_SECRET").expect("SNYK_WEBHOOK_SECRET must be set");
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    
    mac.update(payload);
    let signature_bytes = Vec::from_hex(&signature).unwrap();
    let res= mac.verify_slice(&signature_bytes);
    match res {
        Ok(()) => return true,
        Err(_) => return false,
    }
}


#[rocket::async_trait]
impl<'r> FromData<'r> for Payload {
    type Error = SignatureError;

    async fn from_data(request: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        let signature =
            request.headers().get_one("x-hub-signature").unwrap().split("=").last();
        
        match signature {
            Some(sign) => {
                let mut d = data.open("256 kB".parse().unwrap());
                let mut content = String::new();
                d.read_to_string(&mut content)
                    .await
                    .expect("Could not read content");
                let event: NewSnykEventDto = serde_json::from_str(&content).unwrap();
                let valid = verify_hmac(content.as_bytes(), sign.to_string());
                data::Outcome::Success(Payload { valid, event } )
            },
            None => data::Outcome::Failure((Status::BadRequest, SignatureError::Missing)),
        }
    }
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/store", data = "<input>")]
async fn store(input: Payload) -> status::Custom<content::RawJson<&'static str>> {
    let new_snyk_event: NewSnykEvent = from_new_snyk_event_dto(input.event);
    let mut conn = snyk_webhook_api::establish_connection();
    if input.valid {
        println!("Webhook validation passed, adding event");
        create_snyk_event(new_snyk_event, &mut conn)
             .map(|snyk_event| Json(snyk_event))
             .expect("msg");
        return status::Custom(
            Status::Accepted,
            content::RawJson("Success")
        );
    }
    println!("Webhook validation failed, event rejected");
    status::Custom(
        Status::BadRequest,
        content::RawJson("Invalid signature")
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, store])
}
