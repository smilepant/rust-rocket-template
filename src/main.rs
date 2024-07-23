// -----------------
mod models;
mod routes;
// -----------------

use rocket::{Build, Rocket};
use rocket_okapi::mount_endpoints_and_merged_docs;
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
        .mount("/rapidoc/", routes::rapidoc())
        .mount("/swagger", routes::swagger_ui());

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        building_rocket, "/v1".to_owned(), openapi_settings,
        "/api" => routes::get_routes_and_docs(&openapi_settings),
    };

    building_rocket
}
