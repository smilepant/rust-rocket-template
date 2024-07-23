use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};
use rocket::form::FromForm;
#[derive(Serialize, Deserialize, JsonSchema, FromForm)]
pub struct Post {
    pub post_id: u64,
    pub title: String,
    pub summary: Option<String>,
}
