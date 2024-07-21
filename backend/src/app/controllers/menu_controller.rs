use crate::config::application::Config;
use crate::app::models::menu::{ Menu, MenuForm };
use crate::db::schema::menus::dsl::*;
use crate::app::controllers::{ get_content_type, is_csrf_token_valid, error_controller, prepare_tera_context };
use crate::app::middlewares::auth::AuthState;
use crate::{ render_json, render_form, crud };
use barkeel_lib::app::pagination::{ PaginationQuery, Pagination, PaginationTrait };
use barkeel_lib::app::http::response::Response;
use diesel::prelude::*;
use std::sync::Arc;
use tera::Tera;
use axum::{  Extension, extract::{Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use validator::{Validate, ValidationErrors};
use inflector::Inflector;

crud!(menus, Menu, MenuForm);

pub async fn create(Extension(current_user): Extension<AuthState>, headers: HeaderMap, State(config): State<Arc<Config>>, Form(payload): Form<MenuForm>) -> impl IntoResponse {
    if is_csrf_token_valid(headers.clone(), config.clone(), payload.clone().csrf_token) {
        match payload.validate() {
            Ok(_) => {
                let _inserted_record: Menu = diesel::insert_into(menus)
                .values(name.eq(payload.name))
                .get_result(&mut config.database.pool.get().unwrap())
                .expect("Error inserting data");
                let _ = Redirect::to("/menus");
                let serialized = serde_json::to_string(&"Menu created").unwrap();
                render_json!(StatusCode::OK, serialized)
            },
            Err(e) => {
                let config_ref = config.as_ref();
                let form = payload.build_edit_form(config_ref, headers, "/menus");
                render_form!(form, config, current_user, Some(e.clone()))
            }
        }
    } else {
        let serialized = serde_json::to_string(&"Invalid CSRF token").unwrap();
        render_json!(StatusCode::BAD_REQUEST, serialized) 
    }
}

pub async fn update(Extension(current_user): Extension<AuthState>, headers: HeaderMap, State(config): State<Arc<Config>>, Path(param_id): Path<i32>, Form(payload): Form<MenuForm>) -> impl IntoResponse {
    if is_csrf_token_valid(headers.clone(), config.clone(), payload.clone().csrf_token) {
        match payload.validate() {
            Ok(_) => {
                let _updated_record: Menu = diesel::update(menus)
                    .filter(id.eq(param_id))
                    .set(name.eq(payload.name))
                    .get_result(&mut config.database.pool.get().unwrap())
                    .expect("Error updating data");
                let _ = Redirect::to("/menus");
                let serialized = serde_json::to_string(&"Menu updated").unwrap();
                render_json!(StatusCode::OK, serialized)
            },
            Err(e) => {
                let config_ref = config.as_ref();
                let form = payload.build_edit_form(config_ref, headers, "/menus");
                render_form!(form, config, current_user, Some(e.clone()))
            }
        }
    } else {
        let serialized = serde_json::to_string(&"Invaid CSRF token").unwrap();
        render_json!(StatusCode::BAD_REQUEST, serialized) 
    }
}
