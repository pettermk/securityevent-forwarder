#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::model::SnykEvent;
use crate::model::NewSnykEvent;
use crate::schema::snyk_events;
use crate::schema::snyk_events::dsl::*;

pub fn create_snyk_event(new_snyk_event: NewSnykEvent, conn: &PgConnection) -> QueryResult<SnykEvent> {
    diesel::insert_into(snyk_events::table)
        .values(&new_post)
        .get_result(conn)
}