use crate::config::application::Config;
use crate::app::models::article::{ Article, ArticleForm, ArticleInsertValues, ArticleUpdateValues };
use crate::db::schema::articles::{self, dsl::*};
use crate::app::models::user::User;
use crate::app::controllers::{ CrudTrait, get_content_type, is_csrf_token_valid, error_controller, prepare_tera_context };
use crate::app::middlewares::auth::AuthState;
use barkeel_lib::storage::{local_storage::LocalStorage, FileStorage};
use barkeel_lib::utils::slugify;
use barkeel_lib::app::http::response::Response;
use barkeel_lib::app::pagination::{ PaginationQuery, Pagination, PaginationTrait };
use diesel::prelude::*;
use std::fs;
use std::env;
use std::sync::Arc;
use tera::Tera;
use chrono::Utc;
use validator::{Validate, ValidationErrors};
use axum::{ Extension, extract::{Multipart, Path, State, Query}, response::{ IntoResponse, Redirect }, http::{ HeaderMap, StatusCode }, Form};
use crate::crud;
use inflector::Inflector;

pub struct ArticleController;

impl CrudTrait for ArticleController {
    fn index_view() -> String {
        "../views/article/index.html".to_string()
    }
}

crud!(articles, Article, ArticleForm, ArticleController);


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