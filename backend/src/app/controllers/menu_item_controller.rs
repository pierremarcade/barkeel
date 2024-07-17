use crate::config::application::Config;
use crate::app::models::menu_item::{ MenuItem, MenuItemForm };
use crate::db::schema::menu_items::dsl::*;
use diesel::prelude::*;
use std::sync::Arc;
use tera::Tera;
use axum::{  Extension, extract::{Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use crate::app::controllers::{ get_content_type, is_csrf_token_valid, error_controller, prepare_tera_context };
use crate::app::middlewares::auth::AuthState;
use barkeel_lib::app::pagination::{ PaginationQuery, Pagination, PaginationTrait };
use barkeel_lib::app::http::response::Response;
use validator::{Validate, ValidationErrors};
use crate::{ render_json, render_form, partial_crud };
use inflector::Inflector;

partial_crud!(menu_items, MenuItem);

pub async fn create(Extension(current_user): Extension<AuthState>, headers: HeaderMap, State(config): State<Arc<Config>>, Form(payload): Form<MenuItemForm>) -> impl IntoResponse  {
    if is_csrf_token_valid(headers.clone(), config.clone(), payload.clone().csrf_token) {
        match payload.validate() {
            Ok(_) => {
                let _inserted_record: MenuItem = diesel::insert_into(menu_items)
                .values((menu_id.eq(payload.menu_id), article_id.eq(payload.article_id), label.eq(payload.label), position.eq(payload.position)))
                .get_result(&mut config.database.pool.get().unwrap())
                .expect("Error inserting data");
                let _ = Redirect::to("/menu-items");
                let serialized = serde_json::to_string(&"menu item created").unwrap();
                render_json!(StatusCode::OK, serialized)
            },
            Err(e) => {
                let config_ref = config.as_ref();
                let form = payload.build_edit_form(config_ref, headers, "/menu-items");
                render_form!(form, config, current_user, Some(e.clone()))
            }
        }
    } else {
        let serialized = serde_json::to_string(&"Invaid CSRF token").unwrap();
        render_json!(StatusCode::BAD_REQUEST, serialized) 
    }
}

pub async fn update(Extension(current_user): Extension<AuthState>, headers: HeaderMap, State(config): State<Arc<Config>>, Path(param_id): Path<i32>, Form(payload): Form<MenuItemForm>) -> impl IntoResponse {
    if is_csrf_token_valid(headers.clone(), config.clone(), payload.clone().csrf_token) {
        match payload.validate() {
            Ok(_) => {
                let _updated_record: MenuItem = diesel::update(menu_items)
                    .filter(id.eq(param_id))
                    .set((menu_id.eq(payload.menu_id), article_id.eq(payload.article_id), label.eq(payload.label), position.eq(payload.position)))
                    .get_result(&mut config.database.pool.get().unwrap())
                    .expect("Error updating data");
                let _ = Redirect::to("/menu-items");
                let serialized = serde_json::to_string(&"menu item updated").unwrap();
                render_json!(StatusCode::OK, serialized)
            },
            Err(e) => {
                let config_ref = config.as_ref();
                let form = payload.build_edit_form(config_ref, headers, "/menu-items");
                render_form!(form, config, current_user, Some(e.clone()))
            }
        }
    } else {
        let serialized = serde_json::to_string(&"Invaid CSRF token").unwrap();
        render_json!(StatusCode::BAD_REQUEST, serialized) 
    }
}
