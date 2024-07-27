use rocket_okapi::okapi::schemars::{self, JsonSchema};
use rocket::serde::{json::Json, Deserialize, Serialize};

use rocket_validation::Validate;
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[validate(length(min = 5))]
    pub username: String,
    
    #[validate(email)]
    pub email: Option<String>,

    #[validate(range(min = 1))]
    pub user_id: u64,
}



