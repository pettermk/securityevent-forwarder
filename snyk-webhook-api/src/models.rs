extern crate rocket;
use rocket::serde::{json::{Value}, Deserialize};
use diesel::{prelude::*};
use chrono::{NaiveDateTime, DateTime};
use crate::schema::snyk_events;


#[derive(Queryable)]
pub struct SnykEvent {
    pub id: i32,
    pub ts: NaiveDateTime,
    pub org: Value,
    pub project: Value,
    pub new_issues: Vec<Option<Value>>,
    pub removed_issues: Vec<Option<Value>>,
}


// #[derive(Debug, PartialEq, Eq, Deserialize)]
#[derive(Insertable, Debug, PartialEq, Eq, Deserialize)]
#[diesel(table_name=snyk_events)]
pub struct NewSnykEvent {
    #[serde(with = "json_time")]
    pub ts: NaiveDateTime,
    pub org: Value,
    pub project: Value,
    pub new_issues: Vec<Option<Value>>,
    pub removed_issues: Vec<Option<Value>>,
}

// pub fn time_to_json(t: NaiveDateTime) -> String {
// 	DateTime::<Utc>::from_utc(t, Utc).to_rfc3339()
// }

mod json_time {
	use super::*;
	use serde::{Deserialize, Deserializer, de::Error};
    // use serde::{Serialize, Serializer};

	// pub fn serialize<S: Serializer>(time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error> {
	// 	time_to_json(time.clone()).serialize(serializer)
	// }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDateTime, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        Ok(DateTime::parse_from_rfc3339(&time).map_err(D::Error::custom)?.naive_utc())
    }
}