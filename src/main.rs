// ---------------------------
mod error_handler;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;
use std::collections::HashMap;

// --------------------------
use crate::error_handler::get_catchers;
use rocket::{catchers, Build, Rocket};
use rocket_okapi::mount_endpoints_and_merged_docs;
use std::sync::Mutex;
use utils::utils::UserMap;

#[rocket::main]
async fn main() {
    let launch_result = create_server().launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

pub fn create_server() -> Rocket<Build> {
    let mut building_rocket = rocket::build()
        // .register("/", catchers![rocket_validation::validation_catcher])
        .register("/", get_catchers())
        .mount("/rapidoc/", routes::rapidoc())
        .mount("/swagger", routes::swagger_ui())
        .manage(Mutex::new(HashMap::new()) as UserMap);
    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        building_rocket, "/v1".to_owned(), openapi_settings,
        "/api" => routes::get_routes_and_docs(&openapi_settings)
    };

    building_rocket
}
