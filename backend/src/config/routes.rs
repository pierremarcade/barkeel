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
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(error_controller::handle_timeout_error))
                    .timeout(Duration::from_secs(30))
            )
            .fallback(error_controller::handler_404)
            .route("/public/*path", get(index_controller::handle_assets))
}
