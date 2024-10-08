use axum::{ routing::get, Router };
use barkeel_lib::app::Config;
use crate::app::controllers::api::*;

//Add here new route
pub fn routes(_config: Config) -> Router<Config> {
    let api_routes = Router::new()
        .route("/articles", get(v1::article_controller::index))
        .route("/articles/:slug", get(v1::article_controller::show))
        .route("/articles/search/:query", get(v1::article_controller::search))
        .route("/menus", get(v1::menu_controller::index));
    Router::new().nest("/v1", api_routes)
}



