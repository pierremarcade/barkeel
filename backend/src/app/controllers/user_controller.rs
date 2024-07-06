use crate::config::application::Config;
use crate::app::models::user::{ User, UserForm, UserFormEdit };
use crate::db::schema::users::dsl::*;
use crate::app::utils::{ get_content_type, csrf_token_is_valid };
use crate::app::controllers::error_controller;
use crate::app::middlewares::auth::AuthState;
use crate::app::utils::template::prepare_tera_context;
use barkeel_lib::app::pagination::{ PaginationQuery, Pagination, PaginationTrait };
use barkeel_lib::app::http::response::Response;
use diesel::prelude::*;
use std::sync::Arc;
use tera::Tera;
use axum::{ Extension, extract::{Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form };
use crate::{render_html, render_json, get_total};

pub async fn index(Extension(current_user): Extension<AuthState>, Query(pagination_query): Query<PaginationQuery>, headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let total_results: i64 = get_total!(config, users);
    let pagination = Pagination::new(pagination_query, total_results);
    match users.limit(pagination.per_page as i64).offset(pagination.offset as i64).load::<User>(&mut config.database.pool.get().unwrap()) {
        Ok(results) => {
            if get_content_type(headers) == "application/json" {
                render_json!(config, results)
            } else {    
                let mut context = prepare_tera_context(current_user).await;
                context.insert("title", "User");
                context.insert("base_url", "/users");
                context.insert("description", "A list of all the users.");
                context.insert("datas", &results);
                context.insert("total_pages", &pagination.total_pages);
                context.insert("current_page", &pagination.current_page);
                context.insert("current_page_string", &pagination.current_page.to_string());
                context.insert("offset", &pagination.offset);
                context.insert("per_page", &pagination.per_page);
                context.insert("page_numbers", &pagination.generate_page_numbers());
                let tera: &mut tera::Tera = &mut config.template.clone();
                let _ = tera.add_raw_template("user/index.html", include_str!("../views/user/index.html"));
                let rendered = tera.render("user/index.html", &context);
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
    match users.find(param_id).first::<User>(&mut config.database.pool.get().unwrap()) {
        Ok(result) => {
            tera.add_raw_template("user/show.html", include_str!("../views/user/show.html")).unwrap();
            let mut context = prepare_tera_context(current_user).await;
            context.insert("data", &result);
            context.insert("title", "User");
            context.insert("description", "User's Detail");
            let rendered = tera.render("user/show.html", &context).unwrap();
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
    tera.add_raw_template("user/form.html", include_str!("../views/user/form.html")).unwrap();

    let mut context = prepare_tera_context(current_user).await;
    let config_ref = config.as_ref();
    context.insert("data",&UserForm::new().build_form(config_ref, headers, "/users"));

    let rendered = tera.render("user/form.html", &context).unwrap();
    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
}

pub async fn create(headers: HeaderMap, State(config): State<Arc<Config>>, Form(payload): Form<UserFormEdit>) -> Redirect {
    if csrf_token_is_valid(headers, config.clone(), payload.csrf_token) {
        let _inserted_record: User = diesel::insert_into(users)
            .values((name.eq(payload.name), email.eq(payload.email), password.eq(payload.password), role_id.eq(payload.role_id), session_token.eq(payload.session_token)))
            .get_result(&mut config.database.pool.get().unwrap())
            .expect("Error inserting data");
    }
    Redirect::to("/users") 
}

pub async fn edit(Extension(current_user): Extension<AuthState>, headers: HeaderMap, Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("user/form.html", include_str!("../views/user/form.html")).unwrap();
    let result = users
        .find(param_id)
        .first::<User>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");

    let mut context = prepare_tera_context(current_user).await;
    let config_ref = config.as_ref();
    context.insert("data", &result.build_form(config_ref, headers, format!("/users/{}", param_id).as_str()));

    let rendered = tera.render("user/form.html", &context).unwrap();
    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
}

pub async fn update(headers: HeaderMap, State(config): State<Arc<Config>>, Path(param_id): Path<i32>, Form(payload): Form<UserFormEdit>) -> Redirect {
    if csrf_token_is_valid(headers, config.clone(), payload.csrf_token) {
        let _updated_record: User = diesel::update(users)
            .filter(id.eq(param_id))
            .set((name.eq(payload.name), email.eq(payload.email), password.eq(payload.password), role_id.eq(payload.role_id), session_token.eq(payload.session_token)))
            .get_result(&mut config.database.pool.get().unwrap())
            .expect("Error updating data");
    }
    Redirect::to("/users") 
}

pub async fn delete(Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> Redirect {
    diesel::delete(users)
        .filter(id.eq(param_id))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Error deleting data");
    Redirect::to("/users") 
}
