#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::models::SnykEvent;
use crate::models::NewSnykEvent;
use crate::schema::snyk_events;

pub fn create_snyk_event(new_snyk_event: NewSnykEvent, conn: &mut PgConnection) -> QueryResult<SnykEvent> {
    diesel::insert_into(snyk_events::table)
        .values(&new_snyk_event)
        .get_result(conn)
}
