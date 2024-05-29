use axum::{ extract::{Path, State}, response::Json };
use crate::config::application::Config;
use crate::app::models::article::Article;
use crate::db::schema::articles::dsl::*;
use diesel::prelude::*;
use std::sync::Arc;

pub async fn index(State(config): State<Arc<Config>>) -> Json<String> {
    let results = articles
        .load::<Article>(&mut config.database.pool.get().unwrap())
        .expect("Error loading datas");
    let serialized = serde_json::to_string(&results).unwrap();
    Json(serialized)
}

pub async fn show(Path(other_slug): Path<String>, State(config): State<Arc<Config>>) -> Json<String> {
    let result = articles
        .filter(slug.eq(other_slug))
        .first::<Article>(&mut config.database.pool.get().unwrap())
        .expect("Error loading data");
    let serialized = serde_json::to_string(&result).unwrap();
    Json(serialized)
}
