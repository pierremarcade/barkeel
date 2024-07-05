use axum::{ 
    routing::{get, post, delete}, 
    Router,
    error_handling::HandleErrorLayer
};
use std::sync::Arc;
use crate::config::application::Config;
use crate::app::controllers::*;
use std::time::Duration;
use tower::ServiceBuilder;
use inflector::Inflector;


macro_rules! resource_routes {
    ($router:ident, $model:ident) => {
        {
            let class_name =  stringify!($model).replace("_controller", "").to_string().to_kebab_case().to_plural();
            $router.route(format!("/{}", class_name).as_str(), get($model::index))
            .route(format!("/{}/new", class_name).as_str(), get($model::new))
            .route(format!("/{}/:id", class_name).as_str(), get($model::show))
            .route(format!("/{}/:id", class_name).as_str(), delete($model::delete))
            .route(format!("/{}/:id/edit", class_name).as_str(), get($model::edit))
            .route(format!("/{}", class_name).as_str(), post($model::create))
            .route(format!("/{}/:id", class_name).as_str(), post($model::update))
        }  
    };
}

//Add here new route
pub fn routes(config: Arc<Config>) -> Router<Arc<Config>> {
    let router = Router::new()
        .route("/", get(index_controller::index))
        .route("/logout", get(auth_controller::get::logout));
    let router = resource_routes!(router, menu_item_controller);
    let router = resource_routes!(router, menu_controller);
        resource_routes!(router, article_controller)
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



