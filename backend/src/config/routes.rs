use axum::{ 
    routing::{get, post, patch, delete}, 
    Router,
    error_handling::HandleErrorLayer
};
use std::sync::Arc;
use crate::config::application::Config;
use crate::app::controllers::*;
use std::time::Duration;
use tower::ServiceBuilder;

//Add here new route
pub fn routes(config: Arc<Config>) -> Router<Arc<Config>> {
    Router::new()
            
		    // .route("/users", get(user_controller::index))
            // .route("/users/new", get(user_controller::new))
            // .route("/users/:id", get(user_controller::show))
            // .route("/users/:id/edit", get(user_controller::edit))
            // .route("/books/:id", patch(book_controller::update))
            
            .route("/", get(index_controller::index))
            .route("/logout", get(auth_controller::get::logout))
            .route("/menu-items", get(menu_item_controller::index))
            .route("/menu-items/new", get(menu_item_controller::new))
            .route("/menu-items/:id", get(menu_item_controller::show))
            .route("/menu-items/:id", delete(menu_item_controller::delete))
            .route("/menu-items/:id/edit", get(menu_item_controller::edit))
            .route("/menu-items", post(menu_item_controller::create))
            .route("/menu-items/:id", post(menu_item_controller::update))
            .route("/menus", get(menu_controller::index))
            .route("/menus/new", get(menu_controller::new))
            .route("/menus/:id", get(menu_controller::show))
            .route("/menus/:id", delete(menu_controller::delete))
            .route("/menus/:id/edit", get(menu_controller::edit))
            .route("/menus", post(menu_controller::create))
            .route("/menus/:id", post(menu_controller::update))
            .route("/articles", get(article_controller::index))
            .route("/articles/new", get(article_controller::new))
            .route("/articles/:id", get(article_controller::show))
            .route("/articles/:id", delete(article_controller::delete))
            .route("/articles/:id/edit", get(article_controller::edit))
            .route("/articles", post(article_controller::create))
            .route("/articles/:id", post(article_controller::update))
            .layer(axum::middleware::from_fn(move |req, next| {
                crate::app::middlewares::auth::auth(config.clone(), req, next)
            }))
            .route("/login", get(auth_controller::get::login))
            .route("/login", post(auth_controller::post::login))
            .route("/api/v1/articles", get(api::v1::article_controller::index))
            .route("/api/v1/articles/:slug", get(api::v1::article_controller::show))
            .route("/api/v1/articles/search/:query", get(api::v1::article_controller::search))
            .route("/api/v1/menus", get(api::v1::menu_controller::index))
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(error_controller::handle_timeout_error))
                    .timeout(Duration::from_secs(30))
            )
            .fallback(error_controller::handler_404)
            .route("/public/*path", get(index_controller::handle_assets))
}
