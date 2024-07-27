use rocket::{get, post, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::middleware::auth::Authenticated;
use crate::models::user::User;
use crate::utils::utils::UserMap;

#[openapi(tag = "Users")]
#[post("/", data = "<user>")]
pub async fn create_user(user: Json<User>, user_map: &State<UserMap>) -> Result<Json<User>, Status> {
    let mut map = user_map.lock().unwrap(); // Lock the Mutex to get access to the HashMap
    if map.contains_key(&user.user_id) {
        return Err(Status::Conflict); // User already exists
    }
    map.insert(user.user_id, user.0.clone()); // Insert the cloned User
    Ok(Json(user.0)) // Return the User wrapped in Json
}

#[openapi(tag = "Users")]
#[get("/")]
pub async fn get_all_users(user_map: &State<UserMap>) -> Json<Vec<User>> {
    let map = user_map.lock().unwrap(); // Lock the Mutex to get access to the HashMap
    let users: Vec<User> = map.values().cloned().collect(); // Clone the values
    Json(users) // Return the list of users wrapped in Json
}

#[openapi(tag = "Users")]
#[get("/<id>")]
pub async fn get_user(id: u64, user_map: &State<UserMap>) -> Result<Json<User>, Status> {
    let map = user_map.lock().unwrap(); // Lock the Mutex to get access to the HashMap
    if let Some(user) = map.get(&id) {
        Ok(Json(user.clone())) // Return a clone of the User wrapped in Json
    } else {
        Err(Status::NotFound)
    }
}

#[openapi(tag = "Users")]
#[get("/example?<user_id>&<name>&<email>")]
pub async fn get_user_by_name(user_id: Option<u64>, name: Option<String>, email: Option<String>, user_map: &State<UserMap>) -> Result<Json<User>, Status> {
    let map = user_map.lock().unwrap(); // Lock the Mutex to get access to the HashMap

    if let Some(id) = user_id {
        if let Some(user) = map.get(&id) {
            return Ok(Json(user.clone())); // Return a clone of the User wrapped in Json
        }
    }

    for user in map.values() {
        if let Some(ref n) = name {
            if &user.username == n {
                return Ok(Json(user.clone())); // Return a clone of the User wrapped in Json
            }
        }
        if let Some(ref e) = email {
            if user.email.as_deref() == Some(e) {
                return Ok(Json(user.clone())); // Return a clone of the User wrapped in Json
            }
        }
    }

    Err(Status::NotFound)
}

#[openapi(skip)]
#[get("/hidden")]
pub fn hidden() -> Json<&'static str> {
    Json("Hidden from swagger!")
}

#[openapi(tag = "Users")]
#[get("/protected")]
fn protected(_auth: Authenticated) -> &'static str {
    "You are authorized!"
}

pub fn get_user_routes(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_all_users, get_user, get_user_by_name, create_user, hidden, protected]
}
