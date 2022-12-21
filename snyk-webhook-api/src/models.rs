use diesel::prelude::*;
use rocket::serde::json::Value;
use crate::schema::snyk_events;
use std::{time::SystemTime, collections::HashMap};

#[derive(Queryable)]
pub struct SnykEvent {
    pub id: i32,
    pub ts: SystemTime,
    pub org: HashMap<String, Value>,
    pub project: HashMap<String, Value>,
    pub new_issues: Vec<HashMap<String, Value>>,
    pub removed_issues: Vec<HashMap<String, Value>>,
}

#[derive(Insertable)]
#[diesel(table_name=snyk_events)]
pub struct NewSnykEvent {
    pub org: HashMap<String, Value>,
    pub project: HashMap<String, Value>,
    pub new_issues: Vec<HashMap<String, Value>>,
    pub removed_issues: Vec<HashMap<String, Value>>,
}