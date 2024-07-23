mod posts;
mod route_list;
mod users;
use rocket::Route;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
pub fn get_routes() -> Vec<Route> {
    let openapi_routes = route_list::get_openapi_routes();
    openapi_routes
}

pub fn swagger_ui() -> Vec<Route> {
    make_swagger_ui(&SwaggerUIConfig {
        url: "../openapi.json".to_owned(),
        ..Default::default()
    })
    .into()
}

pub fn rapidoc() -> Vec<Route> {
    make_rapidoc(&RapiDocConfig {
        general: GeneralConfig {
            spec_urls: vec![UrlObject::new("General", "../openapi.json")],
            ..Default::default()
        },
        hide_show: HideShowConfig {
            allow_spec_url_load: false,
            allow_spec_file_load: false,
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}
