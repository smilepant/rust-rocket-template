pub mod posts;
pub mod users;
use rocket::Route;
use rocket_okapi::get_nested_endpoints_and_docs;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::rapidoc::{
    make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig, Theme, UiConfig,
};
use rocket_okapi::settings::{OpenApiSettings, UrlObject};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    get_nested_endpoints_and_docs! {
        "/users" => users::get_user_routes(settings),
        "/post" => posts::get_post_routes(settings),
    }
}

pub fn swagger_ui() -> Vec<Route> {
    make_swagger_ui(&SwaggerUIConfig {
        url: "../v1/openapi.json".to_owned(),
        ..Default::default()
    })
    .into()
}

pub fn rapidoc() -> Vec<Route> {
    make_rapidoc(&RapiDocConfig {
        title: Some("My special documentation | RapiDoc".to_owned()),
        ui: UiConfig {
            theme: Theme::Dark,
            ..Default::default()
        },
        general: GeneralConfig {
            spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
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
