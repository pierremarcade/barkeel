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
use validator::{Validate,  ValidationErrors, ValidationError};
use crate::{render_html, render_json, get_total};

pub async fn index(Extension(current_user): Extension<AuthState>, Query(pagination_query): Query<PaginationQuery>, headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let total_results: i64 = get_total!(config, menu_items);
    let pagination = Pagination::new(pagination_query, total_results);
    match menu_items.limit(pagination.per_page as i64).offset(pagination.offset as i64).load::<MenuItem>(&mut config.database.pool.get().unwrap()) {
        Ok(results) => {
            if get_content_type(headers) == "application/json" {
                render_json!(StatusCode::OK, results)
            } else {
                let mut context = prepare_tera_context(current_user).await;
                context.insert("title", "MenuItem");
                context.insert("base_url", "/menu-items");
                context.insert("description", "A list of all the menu_items.");
                context.insert("datas", &results);
                context.insert("total_pages", &pagination.total_pages);
                context.insert("current_page", &pagination.current_page);
                context.insert("current_page_string", &pagination.current_page.to_string());
                context.insert("offset", &pagination.offset);
                context.insert("per_page", &pagination.per_page);
                context.insert("page_numbers", &pagination.generate_page_numbers()); 
                let tera: &mut tera::Tera = &mut config.template.clone();
                let _ = tera.add_raw_template("menu_item/index.html", include_str!("../views/menu_item/index.html"));
                let rendered = tera.render("menu_item/index.html", &context);
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
    match menu_items.find(param_id).first::<MenuItem>(&mut config.database.pool.get().unwrap()) {
        Ok(result) => {
            tera.add_raw_template("menu_item/show.html", include_str!("../views/menu_item/show.html")).unwrap();
            let mut context = prepare_tera_context(current_user).await;
            context.insert("data", &result);
            context.insert("title", "MenuItem");
            context.insert("description", "MenuItem's Detail");
            let rendered = tera.render("menu_item/show.html", &context).unwrap();
            Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
        },
        _ => {
            error_controller::render_404(config)
        }
    }
}

pub async fn new(Extension(current_user): Extension<AuthState>, headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("menu_item/form.html", include_str!("../views/menu_item/form.html")).unwrap();

    let mut context = prepare_tera_context(current_user).await;
    let config_ref = config.as_ref();
    context.insert("errors_message", "");
    context.insert("data",&MenuItem::build_create_form(config_ref, headers, "/menu-items"));

    let rendered = tera.render("menu_item/form.html", &context).unwrap();
    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
}

pub async fn create(headers: HeaderMap, State(config): State<Arc<Config>>, Form(payload): Form<MenuItemForm>) -> impl IntoResponse  {
    if is_csrf_token_valid(headers, config.clone(), payload.clone().csrf_token) {
        match payload.validate() {
            Ok(_) => {
                let _inserted_record: MenuItem = diesel::insert_into(menu_items)
                .values((menu_id.eq(payload.menu_id), article_id.eq(payload.article_id), label.eq(payload.label), position.eq(payload.position)))
                .get_result(&mut config.database.pool.get().unwrap())
                .expect("Error inserting data");
                let serialized = serde_json::to_string(&"menu item created").unwrap();
                render_json!(StatusCode::OK, serialized)
            },
            Err(e) => {
                let serialized = serde_json::to_string(&e).unwrap();
                render_json!(StatusCode::BAD_REQUEST, serialized)
            }
        };
    } else {
        let serialized = serde_json::to_string(&"Invalid csrf Token").unwrap();
        render_json!(StatusCode::BAD_REQUEST, serialized)
    }
}

pub async fn edit(Extension(current_user): Extension<AuthState>, headers: HeaderMap, Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("menu_item/form.html", include_str!("../views/menu_item/form.html")).unwrap();
    let result = menu_items
        .find(param_id)
        .first::<MenuItem>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");

    let mut context = prepare_tera_context(current_user).await;
    let config_ref = config.as_ref();
    context.insert("data", &result.build_edit_form(config_ref, headers, format!("/menu-items/{}", param_id).as_str()));

    let rendered = tera.render("menu_item/form.html", &context).unwrap();
    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
}

pub async fn update(headers: HeaderMap, State(config): State<Arc<Config>>, Path(param_id): Path<i32>, Form(payload): Form<MenuItemForm>) -> Redirect {
    if is_csrf_token_valid(headers, config.clone(), payload.csrf_token) {
        let _updated_record: MenuItem = diesel::update(menu_items)
            .filter(id.eq(param_id))
            .set((menu_id.eq(payload.menu_id), article_id.eq(payload.article_id), label.eq(payload.label), position.eq(payload.position)))
            .get_result(&mut config.database.pool.get().unwrap())
            .expect("Error updating data");
    }
    Redirect::to("/menu-items") 
}

pub async fn delete(Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> Redirect {
    diesel::delete(menu_items)
        .filter(id.eq(param_id))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Error deleting data");
    Redirect::to("/menu-items") 
}
