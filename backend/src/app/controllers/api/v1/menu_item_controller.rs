use axum::{ extract::{ Path, State}, response::Json };
use crate::config::application::Config;
use crate::app::models::menu_item::MenuItem;
use crate::db::schema::menu_items::dsl::*;
use diesel::prelude::*;
use std::sync::{Arc, Mutex};

pub async fn index(State(config): State<Arc<Mutex<Config>>>) -> Json<String> {
    let config_guard = config.lock().unwrap();
    let results = menu_items
        .load::<MenuItem>(&mut config_guard.database.pool.get().unwrap())
        .expect("Error loading datas");
    let serialized = serde_json::to_string(&results).unwrap();
    Json(serialized)
}

pub async fn show(Path(param_id): Path<i32>, State(config): State<Arc<Mutex<Config>>>) -> Json<String> {
    let config_guard = config.lock().unwrap();
    let result = menu_items
        .find(param_id).first::<MenuItem>(&mut config_guard.database.pool.get().unwrap()) 
        .expect("Error loading data");
    let serialized = serde_json::to_string(&result).unwrap();
    Json(serialized)
}

pub async fn create(Json(payload): Json<MenuItem>, State(config): State<Arc<Mutex<Config>>>) -> Json<String> {
    let config_guard = config.lock().unwrap();
    let inserted_record: MenuItem = diesel::insert_into(menu_items)
        .values((menu_id.eq(payload.menu_id), label.eq(payload.label), position.eq(payload.position)))
        .get_result(&mut config_guard.database.pool.get().unwrap())
        .expect("Error inserting data");
    let serialized = serde_json::to_string(&inserted_record).unwrap();
    Json(serialized)
}

pub async fn update(Path(param_id): Path<i32>, Json(payload): Json<MenuItem>, State(config): State<Arc<Mutex<Config>>>) -> Json<String> {
    let config_guard = config.lock().unwrap();
    let updated_record: MenuItem = diesel::update(menu_items)
        .filter(id.eq(param_id))
        .set((menu_id.eq(payload.menu_id), label.eq(payload.label), position.eq(payload.position)))
        .get_result(&mut config_guard.database.pool.get().unwrap())
        .expect("Error updating data");
    let serialized = serde_json::to_string(&updated_record).unwrap();
    Json(serialized)
}

pub async fn delete(Path(param_id): Path<i32>, State(config): State<Arc<Mutex<Config>>>) -> &'static str {
    let config_guard = config.lock().unwrap();
    diesel::delete(menu_items)
        .filter(id.eq(param_id))
        .execute(&mut config_guard.database.pool.get().unwrap())
        .expect("Error deleting data");
    "Data deleted successfully"
}
