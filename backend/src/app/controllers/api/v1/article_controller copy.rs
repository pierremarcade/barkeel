use axum::{ extract::{Path, State}, response::IntoResponse, http::StatusCode };
use crate::app::utils::{ response::Response };
use crate::config::application::Config;
use crate::db::schema::{ menus, menu_items, articles, article_metas};
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

#[derive(Serialize, Deserialize)]
struct ArticleWithMenuAndMeta{
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub homepage: bool,
    pub section_name: String,
    pub desc: Option<String>,
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
    use crate::db::schema::article_metas;
    let (menu_items_table, menus_table, article_table, meta_desc, title_desc) = diesel::alias!(menu_items as menu_items_table, menus as menus_table, articles as article_table, article_metas as meta_desc, article_metas as title_desc);
    let result =  menu_items_table
        .inner_join(article_table)
        .inner_join(menus_table)
        .left_join(meta_desc.on(meta_desc.field(article_metas::article_id).eq(article_table.field(articles::id)).and(meta_desc.field(article_metas::key).eq("description"))))
        //.inner_join(title_desc.on(title_desc.field(article_metas::article_id).eq(article_table.field(articles::id)).and(title_desc.field(article_metas::key).eq("title"))))
        //.select((article_table.field(articles::id), article_table.field(articles::title), article_table.field(articles::content), article_table.field(articles::slug), article_table.field(articles::homepage), menus_table.field(menus::name)))
        .select((article_table.field(articles::id), article_table.field(articles::title), article_table.field(articles::content), article_table.field(articles::slug), article_table.field(articles::homepage), menus_table.field(menus::name), meta_desc.field(article_metas::value)))
        .filter(article_table.field(articles::slug).eq(other_slug))
        .first::<(i32, String, String, String, bool, String, Option<String>)>(&mut config.database.pool.get().unwrap())
        //.first::<ArticleWithMenuAndMeta>(&mut config.database.pool.get().unwrap())
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
