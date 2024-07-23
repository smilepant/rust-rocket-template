use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: u64,
    pub username: String,
    pub email: Option<String>,
}