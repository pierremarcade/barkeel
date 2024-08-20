use axum::{ 
    routing::{get, post, delete}, 
    Router,
    error_handling::HandleErrorLayer
};
use crate::config::application::Config;
use crate::app::controllers::*;
use std::time::Duration;
use tower::ServiceBuilder;
use inflector::Inflector;

//Add here new route
pub fn routes(config: Config) -> Router<Config> {
    let auth_config = config.clone();
    let locale_config = config.clone();
    let router = Router::new()
        .route("/", get(index_controller::index))
        .route("/logout", get(auth_controller::get::logout));
    let router = resource_routes!(router, menu_item_controller);
    let router = resource_routes!(router, menu_controller);
        resource_routes!(router, article_controller)
        .route("/articles/search", get(article_controller::search))
        .route("/articles/upload", post(article_controller::upload))
        .layer(axum::middleware::from_fn(move |req, next| {
            crate::app::middlewares::auth::auth(auth_config.clone(), req, next)
        }))
        .route("/login", get(auth_controller::get::login))
        .route("/login", post(auth_controller::post::login))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(error_controller::handle_timeout_error))
                .timeout(Duration::from_secs(30))
        )
        .layer(axum::middleware::from_fn(move |req, next| {
            crate::app::middlewares::locale::change_locale(locale_config.clone(), req, next)
        }))
        .fallback(error_controller::handler_404)
        .route("/public/*path", get(index_controller::handle_assets))
}



