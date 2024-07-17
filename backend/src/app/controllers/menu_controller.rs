use crate::config::application::Config;
use crate::app::models::menu::{ Menu, MenuForm };
use crate::db::schema::menus::dsl::*;
use crate::app::controllers::{ get_content_type, is_csrf_token_valid, error_controller, prepare_tera_context };
use crate::app::middlewares::auth::AuthState;
use crate::{ render_html, render_json, get_total, render_form };
use barkeel_lib::app::pagination::{ PaginationQuery, Pagination, PaginationTrait };
use barkeel_lib::app::http::response::Response;
use diesel::prelude::*;
use std::sync::Arc;
use tera::Tera;
use axum::{  Extension, extract::{Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use validator::{Validate, ValidationErrors};

pub async fn index(Extension(current_user): Extension<AuthState>, Query(pagination_query): Query<PaginationQuery>, headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let total_results: i64 = get_total!(config, menus);
    let pagination = Pagination::new(pagination_query, total_results);
    match menus.limit(pagination.per_page as i64).offset(pagination.offset as i64).load::<Menu>(&mut config.database.pool.get().unwrap()) {
        Ok(results) => {
            if get_content_type(headers) == "application/json" {
                render_json!(StatusCode::OK, results)
            } else {    
                let mut context = prepare_tera_context(current_user).await;
                context.insert("title", "Menu");
                context.insert("base_url", "/menus");
                context.insert("description", "A list of all the menus.");
                context.insert("datas", &results);
                context.insert("total_pages", &pagination.total_pages);
                context.insert("current_page", &pagination.current_page);
                context.insert("current_page_string", &pagination.current_page.to_string());
                context.insert("offset", &pagination.offset);
                context.insert("per_page", &pagination.per_page);
                context.insert("page_numbers", &pagination.generate_page_numbers());
                let tera: &mut tera::Tera = &mut config.template.clone();
                let _ = tera.add_raw_template("menu/index.html", include_str!("../views/menu/index.html"));
                let rendered = tera.render("menu/index.html", &context);
                render_html!(config, rendered)
            }
        },
        Err(err) => {
            error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string())
        }
    }
}

pub async fn show(Extension(current_user): Extension<AuthState>, Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    match menus.find(param_id).first::<Menu>(&mut config.database.pool.get().unwrap()) {
        Ok(result) => {
            tera.add_raw_template("menu/show.html", include_str!("../views/menu/show.html")).unwrap();
            let mut context = prepare_tera_context(current_user).await;
            context.insert("data", &result);
            context.insert("title", "Menu");
            context.insert("description", "Menu's Detail");
            let rendered = tera.render("menu/show.html", &context).unwrap();
            Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
        },
        _ => {
            error_controller::render_404(config)
        }
    }
}

pub async fn new(Extension(current_user): Extension<AuthState>, headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let config_ref = config.as_ref();
    let form = Menu::build_create_form(config_ref, headers, "/menus");
    render_form!(form, config, current_user, None::<Option<ValidationErrors>>)
}

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

pub async fn edit(Extension(current_user): Extension<AuthState>, headers: HeaderMap, Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let result = menus
        .find(param_id)
        .first::<Menu>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");

    let config_ref = config.as_ref();
    let form = result.build_edit_form(config_ref, headers, format!("/menus/{}", param_id).as_str());
    render_form!(form, config, current_user, None::<Option<ValidationErrors>>)
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

pub async fn delete(Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> Redirect {
    diesel::delete(menus)
        .filter(id.eq(param_id))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Error deleting data");
    Redirect::to("/menus") 
}
