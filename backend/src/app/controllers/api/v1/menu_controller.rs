use axum::{ extract::{ Path, State}, response::Json };
use crate::config::application::Config;
use crate::app::models::menu::Menu;
use crate::db::schema::menus::dsl::*;
use diesel::prelude::*;
use std::sync::Arc;

pub async fn index(State(config): State<Arc<Config>>) -> Json<String> {
    let results = menus
        .load::<Menu>(&mut config.database.pool.get().unwrap())
        .expect("Error loading datas");
    let serialized = serde_json::to_string(&results).unwrap();
    Json(serialized)
}

pub async fn show(Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> Json<String> {
    let result = menus
        .find(param_id).first::<Menu>(&mut config.database.pool.get().unwrap()) 
        .expect("Error loading data");
    let serialized = serde_json::to_string(&result).unwrap();
    Json(serialized)
}

pub async fn create(Json(payload): Json<Menu>, State(config): State<Arc<Config>>) -> Json<String> {
    let inserted_record: Menu = diesel::insert_into(menus)
        .values((name.eq(payload.name), href.eq(payload.href)))
        .get_result(&mut config.database.pool.get().unwrap())
        .expect("Error inserting data");
    let serialized = serde_json::to_string(&inserted_record).unwrap();
    Json(serialized)
}

pub async fn update(Path(param_id): Path<i32>, Json(payload): Json<Menu>, State(config): State<Arc<Config>>) -> Json<String> {
    let updated_record: Menu = diesel::update(menus)
        .filter(id.eq(param_id))
        .set((name.eq(payload.name), href.eq(payload.href)))
        .get_result(&mut config.database.pool.get().unwrap())
        .expect("Error updating data");
    let serialized = serde_json::to_string(&updated_record).unwrap();
    Json(serialized)
}

pub async fn delete(Path(param_id): Path<i32>, State(config): State<Arc<Config>>) -> &'static str {
    diesel::delete(menus)
        .filter(id.eq(param_id))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Error deleting data");
    "Data deleted successfully"
}
