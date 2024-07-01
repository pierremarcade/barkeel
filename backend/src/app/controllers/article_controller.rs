use crate::config::application::Config;
use crate::app::models::article::{ Article, ArticleForm, ArticleFormEdit };
use crate::db::schema::articles::{self, dsl::*};
use crate::app::utils::{ get_content_type, csrf_token_is_valid, response::Response, pagination::{ PaginationQuery, Pagination } };
use crate::app::controllers::error_controller;
use crate::app::middlewares::auth::AuthState;
use crate::app::utils::template::prepare_tera_context;
use crate::app::utils::pagination::PaginationTrait;
use diesel::prelude::*;
use std::sync::Arc;
use tera::Tera;
use chrono::Utc;
use axum::{ Extension, extract::{Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};

pub async fn index(Extension(current_user): Extension<AuthState>, Query(pagination_query): Query<PaginationQuery>, headers: HeaderMap, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let total_results: i64 = get_total(config.clone());
    let pagination = Pagination::new(pagination_query, total_results);
    match articles.limit(pagination.per_page as i64).offset(pagination.offset as i64).load::<Article>(&mut config.database.pool.get().unwrap()) {
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

async fn render_html(current_user: AuthState, config: Arc<Config>, results: Vec<Article>, pagination: Pagination) -> Response<'static> {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    let template_path = "article/index.html";
    let template_content = include_str!("../views/article/index.html");
    let result = tera.add_raw_template(template_path, template_content);
    match result {
        Ok(_) => {},
        Err(err) => {
            return error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string());
        }
    }
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

    let rendered = tera.render("article/index.html", &context);
    match rendered {
        Ok(result) => {
            Response{status_code: StatusCode::OK, content_type: "text/html", datas: result}
        },
        Err(err) => {
            error_controller::handler_error(config, StatusCode::BAD_REQUEST, err.to_string())
        }
    }
}

fn render_json(config: Arc<Config>, results: Vec<Article>) -> Response<'static> {
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
    match articles.count().get_result(&mut config.database.pool.get().unwrap()) {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Error counting articles: {}", e);
            0
        }
    }
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
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("article/form.html", include_str!("../views/article/form.html")).unwrap();
    let mut context = prepare_tera_context(current_user).await;
    let config_ref = config.as_ref();
    let article_from = ArticleForm::new();
    context.insert("data",&article_from.build_form(config_ref, headers, "/articles"));
    let rendered = tera.render("article/form.html", &context).unwrap();
    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
}

pub async fn create(Extension(mut current_user): Extension<AuthState>, headers: HeaderMap, State(config): State<Arc<Config>>, Form(payload): Form<ArticleFormEdit>) -> Redirect {
    if csrf_token_is_valid(headers, config.clone(), payload.csrf_token) {
        if let Some(user) = current_user.get_user().await {
            let _inserted_record: Article = diesel::insert_into(articles)
                .values((title.eq(payload.title), slug.eq(payload.slug),content.eq(payload.content), published_at.eq(Utc::now().naive_utc()), author_id.eq(user.id), homepage.eq(payload.homepage)))
                .get_result(&mut config.database.pool.get().unwrap())
                .expect("Error inserting data");
        }
    }
    Redirect::to("/articles") 
}

pub async fn edit(Extension(current_user): Extension<AuthState>, headers: HeaderMap, Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let mut tera = tera.clone();
    tera.add_raw_template("article/form.html", include_str!("../views/article/form.html")).unwrap();
    let result = articles
        .find(param_id)
        .first::<Article>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");
    let mut context = prepare_tera_context(current_user).await;
    let config_ref = config.as_ref();
    context.insert("data", &result.build_form(config_ref, headers, format!("/articles/{}", param_id).as_str()));
    let rendered = tera.render("article/form.html", &context).unwrap();
    Response{status_code: StatusCode::OK, content_type: "text/html", datas: rendered}
}

pub async fn update(headers: HeaderMap, State(config): State<Arc<Config>>, Path(param_id): Path<i32>, Form(payload): Form<ArticleFormEdit>) -> Redirect {
    if csrf_token_is_valid(headers, config.clone(), payload.csrf_token) {
        let _updated_record: Article = diesel::update(articles)
            .filter(id.eq(param_id))
            .set((title.eq(payload.title), slug.eq(payload.slug), content.eq(payload.content), homepage.eq(payload.homepage)))
            .get_result(&mut config.database.pool.get().unwrap())
            .expect("Error updating data");
    }
    Redirect::to("/articles") 
}

pub async fn delete(Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> Redirect {
    diesel::delete(articles)
        .filter(id.eq(param_id))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Error deleting data");
    Redirect::to("/articles") 
}
