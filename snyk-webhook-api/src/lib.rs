use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::path::PathBuf;


pub fn establish_connection() -> PgConnection {
    match dotenv() {
        Ok(result) => result,
        Err(_) => PathBuf::new(),
    };

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
