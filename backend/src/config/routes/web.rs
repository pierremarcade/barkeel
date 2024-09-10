use axum::{ 
    routing::{ get, post, delete }, 
    Router,
    error_handling::HandleErrorLayer
};
use barkeel_lib::app::Config;
use crate::app::controllers::*;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use inflector::Inflector;

//Add here new route
pub fn routes(config: Config) -> Router<Config> {
    let public_dir = ServeDir::new("src/public");
    let auth_config = config.clone();
    let router = Router::new()
        .route("/", get(index_controller::index));
       
    let router = resource_routes!(router, menu_item_controller);
    let router = resource_routes!(router, menu_controller);
        resource_routes!(router, article_controller)
        .route("/articles/search", get(article_controller::search))
        .route("/articles/upload", post(article_controller::upload))
        .route("/login", get(auth_controller::get::login))
        .layer(axum::middleware::from_fn(move |req, next| {
            crate::app::middlewares::auth::auth(auth_config.clone(), req, next)
        }))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(error_controller::handle_timeout_error))
                .timeout(Duration::from_secs(30))
        )
        .layer(axum::middleware::from_fn(move |req, next| {
            crate::app::middlewares::locale::change_locale(req, next)
        }))
        .layer(axum::middleware::from_fn(move |req, next| {
            crate::app::middlewares::session_token::unique_id_middleware(req, next)
        }))
        .route("/logout", get(auth_controller::get::logout))
        .route("/login", post(auth_controller::post::login))
        .nest_service("/public", public_dir.clone()).fallback_service(public_dir)
        .fallback(error_controller::handler_404)
        
}



