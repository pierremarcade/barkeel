use barkeel_lib::app::Config;
use crate::app::models::article::{ Article, ArticleForm, ArticleInsertValues, ArticleUpdateValues };
use crate::db::schema::articles::{self, dsl::*};
use crate::app::models::user::User;
use crate::app::controllers::{ CrudViewTrait, get_locale, get_content_type, is_csrf_token_valid, error_controller, prepare_tera_context };
use crate::app::middlewares::auth::AuthState;
use barkeel_lib::storage::{local_storage::LocalStorage, FileStorage};
use barkeel_lib::utils::slugify;
use barkeel_lib::app::http::response::Response;
use barkeel_lib::app::pagination::{ RequestQuery, Pagination, PaginationTrait };
use diesel::prelude::*;
use tera::Tera;
use chrono::Utc;
use validator::{Validate, ValidationErrors};
use axum::{ Extension, extract::{Multipart, Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use crate::crud;
use inflector::Inflector;
use std::collections::HashMap;
use crate::config::application::LOCALES;
use fluent_templates::Loader;

type CrudModel = Article;
type CrudForm = ArticleForm;

pub struct ArticleView;
impl CrudViewTrait for ArticleView {
    fn index_view(tera: &mut Tera) -> String {
        let _ = tera.add_raw_template("article_index", include_str!("../views/article/index.html"));
        "article_index".to_string()
    }
}

crud!(articles, ArticleView);

fn insert_values(payload: ArticleForm, current_user: User) -> ArticleInsertValues {
    ArticleInsertValues {
        title: payload.title.clone(),
        slug: slugify(&payload.title.clone()),
        content: payload.content,
        published_at: Utc::now().naive_utc(),
        author_id: Some(current_user.id),
        homepage: payload.homepage
    }
}

fn update_values(payload: ArticleForm, _current_user: User) -> ArticleUpdateValues {
    ArticleUpdateValues {
        title: payload.title.clone(),
        slug: slugify(&payload.title.clone()),
        content: payload.content,
        homepage: payload.homepage
    }
}

pub async fn search(Query(params): Query<HashMap<String, String>>, State(config): State<Config>, headers: HeaderMap) -> impl IntoResponse {
    let locale = get_locale(headers, None);
    let mut query = articles::table.into_boxed();
    if let Some(title_param) = params.get("title") {
        query = query.filter(articles::title.ilike(format!("%{}%", title_param)))
                     .or_filter(articles::content.ilike(format!("%{}%", title_param)));
    }

    if let Some(ids_str) = params.get("ids") {
        let ids: Vec<i32> = ids_str.split(',')
                                    .map(|id_str| id_str.parse().unwrap_or_default())
                                    .collect();
        query = query.filter(articles::id.eq_any(ids));
    }
    let results = query.limit(10)
        .load::<Article>(&mut config.database.pool.get().unwrap())
        .unwrap_or_else(|_| { panic!("{}", LOCALES.lookup(&locale, "error_update").to_string()) });
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