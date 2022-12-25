// pub mod models;
// pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn create_snyk_event(conn: &mut PgConnection, snyk_event: NewSnykEvent) -> SnykEvent {
// 
//     diesel::insert_into(snyk_events::table)
//         .values(&snyk_event)
//         .get_result(conn)
//         .expect("Error saving new post")
// }