
use axum::{ Extension, extract::{ State, Path}, response:: { Html, IntoResponse }, http::{header, HeaderMap, StatusCode} };
use tera::Tera;
use barkeel_lib::app::Config;
use crate::app::models::auth::AuthState;
use crate::app::controllers::prepare_tera_context;

static MAIN_CSS: &str = include_str!("../../public/css/main.css");
static MAIN_JS: &str = include_str!("../../public/js/main.js");
static FAVICON: &str = include_str!("../../public/img/favicon.svg");

pub async fn index(Extension(current_user): Extension<AuthState>, State(config): State<Config>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("index.html", include_str!("../views/index.html")).unwrap();
    let context = prepare_tera_context(current_user).await;
    let rendered = tera.render("index.html", &context).unwrap();
    Html(rendered)
}

pub async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    if path == "css/main.css" {
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
        (StatusCode::OK, headers, MAIN_CSS)
    } else if path == "js/main.js" {
        headers.insert(header::CONTENT_TYPE, "application/javascript".parse().unwrap());
        (StatusCode::OK, headers, MAIN_JS)
    } else if path == "img/favicon.svg" {
        (StatusCode::OK, headers, FAVICON)
    } else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}