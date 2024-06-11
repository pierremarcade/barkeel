use axum::{ extract::{Path, State}, response::IntoResponse, http::StatusCode };
use crate::app::utils::{ response::Response };
use crate::config::application::Config;
use crate::db::schema::{ menus, menu_items, articles};
use crate::db::schema::articles::dsl::*;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Queryable)]
struct ArticleWithMenu{
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub homepage: bool,
    pub section_name: String
}

pub async fn index(State(config): State<Arc<Config>>) -> impl IntoResponse {
    let results = menu_items::table
        .inner_join(articles::table)
        .inner_join(menus::table)
        .select((articles::id, articles::title, articles::slug, articles::content, articles::homepage, menus::name))
        .load::<ArticleWithMenu>(&mut config.database.pool.get().unwrap())
        .expect("Error loading datas");
    let serialized = serde_json::to_string(&results).unwrap();
    Response{status_code: StatusCode::OK, content_type: "application/json", datas: serialized}
}

pub async fn show(Path(other_slug): Path<String>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let result =  menu_items::table
        .inner_join(articles::table)
        .inner_join(menus::table)
        .select((articles::id, articles::title, articles::slug, articles::content, articles::homepage, menus::name))
        .filter(slug.eq(other_slug))
        .first::<ArticleWithMenu>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");

    let serialized = serde_json::to_string(&result).unwrap();
    Response{status_code: StatusCode::OK, content_type: "application/json", datas: serialized}
}


pub async fn search(Path(query): Path<String>, State(config): State<Arc<Config>>) -> impl IntoResponse {
    let results = menu_items::table
        .inner_join(articles::table)
        .inner_join(menus::table)
        .select((articles::id, articles::title, articles::slug, articles::content, articles::homepage, menus::name))
        .filter(articles::title.ilike(format!("%{}%", query.clone())))
        .or_filter(articles::content.ilike(format!("%{}%", query.clone())))
        .or_filter(menus::name.ilike(format!("%{}%", query.clone())))
        .limit(10)
        .load::<ArticleWithMenu>(&mut config.database.pool.get().unwrap())
        .expect("Error loading datas");
    let serialized = serde_json::to_string(&results).unwrap();
    Response{status_code: StatusCode::OK, content_type: "application/json", datas: serialized}
}
