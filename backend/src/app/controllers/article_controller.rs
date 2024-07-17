use crate::config::application::Config;
use crate::app::models::article::{ Article, ArticleForm };
use crate::db::schema::articles::{self, dsl::*};
use crate::app::controllers::{ get_content_type, is_csrf_token_valid, error_controller, prepare_tera_context };
use crate::app::middlewares::auth::AuthState;
use barkeel_lib::storage::{local_storage::LocalStorage, FileStorage};
use barkeel_lib::utils::slugify;
use barkeel_lib::app::http::response::Response;
use barkeel_lib::app::pagination::{ PaginationQuery, Pagination, PaginationTrait };
use diesel::prelude::*;
use std::sync::Arc;
use tera::Tera;
use chrono::Utc;
use validator::{Validate, ValidationErrors};
use axum::{ Extension, extract::{Multipart, Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use crate::{render_html, render_json, get_total};

pub async fn index(Extension(current_user): Extension<AuthState>, Query(pagination_query): Query<PaginationQuery>, headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let total_results: i64 = get_total!(config, articles);
    let pagination = Pagination::new(pagination_query, total_results);
    match articles.limit(pagination.per_page as i64).offset(pagination.offset as i64).load::<Article>(&mut config.database.pool.get().unwrap()) {
        Ok(results) => {
            if get_content_type(headers) == "application/json" {
                render_json!(StatusCode::OK, results)
            } else {    
                let mut context = prepare_tera_context(current_user).await;
                context.insert("title", "Article");
                context.insert("base_url", "/articles");
                context.insert("description", "A list of all the articles.");
                context.insert("datas", &results);
                context.insert("total_pages", &pagination.total_pages);
                context.insert("current_page", &pagination.current_page);
                context.insert("current_page_string", &pagination.current_page.to_string());
                context.insert("offset", &pagination.offset);
                context.insert("per_page", &pagination.per_page);
                context.insert("page_numbers", &pagination.generate_page_numbers());
                let tera: &mut tera::Tera = &mut config.template.clone();
                let _ = tera.add_raw_template("article/index.html", include_str!("../views/article/index.html"));
                let rendered = tera.render("article/index.html", &context);
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
    match articles.find(param_id).first::<Article>(&mut config.database.pool.get().unwrap()) {
        Ok(result) => {
            tera.add_raw_template("article/show.html", include_str!("../views/article/show.html")).unwrap();
            let mut context = prepare_tera_context(current_user).await;
            context.insert("data", &result);
            context.insert("title", "Article");
            context.insert("description", "Article's Detail");
            let rendered = tera.render("article/show.html", &context).unwrap();
            Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
        },
        _ => {
            error_controller::render_404(config)
        }
    }
}

pub async fn new(Extension(current_user): Extension<AuthState>, headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let config_ref = config.as_ref();
    let form = Article::build_create_form(config_ref, headers, "/articles");
    render_form!(form, config, current_user, None::<Option<ValidationErrors>>)
}

pub async fn create(Extension(mut current_user): Extension<AuthState>, headers: HeaderMap, State(config): State<Arc<Config>>, Form(payload): Form<ArticleForm>) -> impl IntoResponse {
    if is_csrf_token_valid(headers.clone(), config.clone(), payload.clone().csrf_token) {
        match payload.validate() {
            Ok(_) => {
                if let Some(user) = current_user.get_user().await {
                    let _inserted_record: Article = diesel::insert_into(articles)
                        .values((title.eq(payload.title.clone()), slug.eq(slugify(&payload.title.clone())),content.eq(payload.content), published_at.eq(Utc::now().naive_utc()), author_id.eq(user.id), homepage.eq(payload.homepage)))
                        .get_result(&mut config.database.pool.get().unwrap())
                        .expect("Error inserting data");
                }
                let _ = Redirect::to("/articles");
                let serialized = serde_json::to_string(&"Article created").unwrap();
                render_json!(StatusCode::OK, serialized)
            },
            Err(e) => {
                let config_ref = config.as_ref();
                let form = payload.build_edit_form(config_ref, headers, "/articles");
                render_form!(form, config, current_user, Some(e.clone()))
            }
        }
    } else {
        let serialized = serde_json::to_string(&"Invalid CSRF token").unwrap();
        render_json!(StatusCode::BAD_REQUEST, serialized) 
    }
}

pub async fn edit(Extension(current_user): Extension<AuthState>, headers: HeaderMap, Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let result = articles
        .find(param_id)
        .first::<Article>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");

    let config_ref = config.as_ref();
    let form = result.build_edit_form(config_ref, headers, format!("/articles/{}", param_id).as_str());
    render_form!(form, config, current_user, None::<Option<ValidationErrors>>)
}

pub async fn update(Extension(current_user): Extension<AuthState>, headers: HeaderMap, State(config): State<Arc<Config>>, Path(param_id): Path<i32>, Form(payload): Form<ArticleForm>) -> impl IntoResponse {
    if is_csrf_token_valid(headers.clone(), config.clone(), payload.clone().csrf_token) {
        match payload.validate() {
            Ok(_) => {
                let _updated_record: Article = diesel::update(articles)
                    .filter(id.eq(param_id))
                    .set((title.eq(payload.title.clone()), slug.eq(slugify(&payload.title.clone())), content.eq(payload.content), homepage.eq(payload.homepage)))
                    .get_result(&mut config.database.pool.get().unwrap())
                    .expect("Error updating data");
                let _ = Redirect::to("/articles");
                let serialized = serde_json::to_string(&"Article updated").unwrap();
                render_json!(StatusCode::OK, serialized)
            },
            Err(e) => {
                let config_ref = config.as_ref();
                let form = payload.build_edit_form(config_ref, headers, "/articles");
                render_form!(form, config, current_user, Some(e.clone()))
            }
        }
    } else {
        let serialized = serde_json::to_string(&"Invaid CSRF token").unwrap();
        render_json!(StatusCode::BAD_REQUEST, serialized) 
    }
}

pub async fn delete(Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> Redirect {
    diesel::delete(articles)
        .filter(id.eq(param_id))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Error deleting data");
    Redirect::to("/articles") 
}

pub async fn search(Path(query): Path<String>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let results = articles::table
        .filter(articles::title.ilike(format!("%{}%", query.clone())))
        .or_filter(articles::content.ilike(format!("%{}%", query.clone())))
        .limit(10)
        .load::<Article>(&mut config.database.pool.get().unwrap())
        .expect("Error loading datas");
    let serialized = serde_json::to_string(&results).unwrap();
    Response{status_code: StatusCode::OK, content_type: "application/json", datas: serialized}
}

pub async fn upload(mut multipart: Multipart) {
    let local_storage = LocalStorage::new("/home/pierre/images/barkeel");
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let _ = local_storage.store(&file_name, &data).await;
    }
}