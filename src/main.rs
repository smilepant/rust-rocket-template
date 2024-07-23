
extern crate rocket;
mod models;
mod routes;

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .mount("/", routes::get_routes())
        .mount("/swagger-ui/", routes::swagger_ui())
        .mount("/rapidoc/", routes::rapidoc())
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}
