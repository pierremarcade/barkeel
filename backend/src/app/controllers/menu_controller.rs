use crate::config::application::Config;
use crate::app::models::menu::{ Menu, MenuForm, MenuFormEdit };
use crate::db::schema::menus::dsl::*;
use diesel::prelude::*;
use std::sync::Arc;
use tera::Tera;
use axum::{ Extension, extract::{Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form };
use crate::app::utils::{ get_content_type, csrf_token_is_valid };
use crate::app::controllers::error_controller;
use crate::app::middlewares::auth::AuthState;
use crate::app::utils::template::prepare_tera_context;
use barkeel_lib::app::pagination::{ PaginationQuery, Pagination, PaginationTrait };
use barkeel_lib::app::http::response::Response;

pub async fn index(Extension(current_user): Extension<AuthState>, Query(pagination_query): Query<PaginationQuery>, headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let total_results: i64 = get_total(config.clone());
    let pagination = Pagination::new(pagination_query, total_results);
    match menus.limit(pagination.per_page as i64).offset(pagination.offset as i64).load::<Menu>(&mut config.database.pool.get().unwrap()) {
        Ok(results) => {
            if get_content_type(headers) == "application/json" {
                render_json(config, results)
            } else {    
                render_html(current_user, config, results, pagination).await
            }
        },
        Err(err) => {
            error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string())
        }
    }
}

async fn render_html(current_user: AuthState, config: Arc<Config>, results: Vec<Menu>, pagination: Pagination) -> Response<'static> {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    let template_path = "menu/index.html";
    let template_content = include_str!("../views/menu/index.html");
    let result = tera.add_raw_template(template_path, template_content);
    match result {
        Ok(_) => {},
        Err(err) => {
            return error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string());
        }
    }
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

    let rendered = tera.render("menu/index.html", &context);
    match rendered {
        Ok(result) => {
            Response{status_code: StatusCode::OK, content_type: "text/html", datas: result}
        },
        Err(err) => {
            error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string())
        }
    }
}

fn render_json(config: Arc<Config>, results: Vec<Menu>) -> Response<'static> {
    match  serde_json::to_string(&results) {
        Ok(serialized) => {
            return Response{status_code: StatusCode::OK, content_type: "application/json", datas: serialized};
        },
        Err(err) => {
            return error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string());
        }
    }
}

fn get_total(config: Arc<Config>) -> i64 {
    match menus.count().get_result(&mut config.database.pool.get().unwrap()) {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Error counting menus: {}", e);
            0 
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
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("menu/form.html", include_str!("../views/menu/form.html")).unwrap();

    let mut context = prepare_tera_context(current_user).await;
    let config_ref = config.as_ref();
    context.insert("data",&MenuForm::new().build_form(config_ref, headers, "/menus"));

    let rendered = tera.render("menu/form.html", &context).unwrap();
    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
}

pub async fn create(headers: HeaderMap, State(config): State<Arc<Config>>, Form(payload): Form<MenuFormEdit>) -> Redirect {
    if csrf_token_is_valid(headers, config.clone(), payload.csrf_token) {
        let _inserted_record: Menu = diesel::insert_into(menus)
            .values(name.eq(payload.name))
            .get_result(&mut config.database.pool.get().unwrap())
            .expect("Error inserting data");
    }
    Redirect::to("/menus") 
}

pub async fn edit(Extension(current_user): Extension<AuthState>, headers: HeaderMap, Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("menu/form.html", include_str!("../views/menu/form.html")).unwrap();
    let result = menus
        .find(param_id)
        .first::<Menu>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");

    let mut context = prepare_tera_context(current_user).await;
    let config_ref = config.as_ref();
    context.insert("data", &result.build_form(config_ref, headers, format!("/menus/{}", param_id).as_str()));

    let rendered = tera.render("menu/form.html", &context).unwrap();
    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
}

pub async fn update(headers: HeaderMap, State(config): State<Arc<Config>>, Path(param_id): Path<i32>, Form(payload): Form<MenuFormEdit>) -> Redirect {
    if csrf_token_is_valid(headers, config.clone(), payload.csrf_token) {
        let _updated_record: Menu = diesel::update(menus)
            .filter(id.eq(param_id))
            .set(name.eq(payload.name))
            .get_result(&mut config.database.pool.get().unwrap())
            .expect("Error updating data");
    }
    Redirect::to("/menus") 
}

pub async fn delete(Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> Redirect {
    diesel::delete(menus)
        .filter(id.eq(param_id))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Error deleting data");
    Redirect::to("/menus") 
}
