use axum::{ extract::{Path, State}, response::IntoResponse, http::StatusCode };
use crate::app::utils::{ response::Response };
use crate::config::application::Config;
use crate::db::schema::{ menus, menu_items, articles::{self, dsl::*}, article_metas};
use crate::app::models::article::{ArticleWithMenu, ArticleWithMenuAndMeta};
use diesel::prelude::*;
use std::sync::Arc;

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
    let (meta_desc, _title_desc) = diesel::alias!(article_metas as meta_desc, article_metas as title_desc);
    
    match menu_items::table
        .inner_join(articles::table)
        .inner_join(menus::table)
        .left_join(meta_desc.on(meta_desc.field(article_metas::article_id).eq(articles::id).and(meta_desc.field(article_metas::key).eq("description"))))
        .select((articles::id, articles::title, articles::slug, articles::content, articles::homepage, menus::name, meta_desc.field(article_metas::value).nullable()))
        .filter(slug.eq(other_slug)).first::<(i32, String, String, String, bool, String, Option<String>)>(&mut config.database.pool.get().unwrap()) {
        Ok(result) => {
            let serialized = serde_json::to_string(&ArticleWithMenuAndMeta::new(result)).unwrap();
            Response{status_code: StatusCode::OK, content_type: "application/json", datas: serialized}
        },
        _ => {
            Response{status_code: StatusCode::NOT_FOUND, content_type: "application/json", datas: "Not found".to_string()}
        }
    }  
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
