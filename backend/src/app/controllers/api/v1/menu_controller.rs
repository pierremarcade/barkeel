use axum::{ extract::{Path, State}, response::{ Json, IntoResponse }, http::StatusCode };
use barkeel_lib::app::Config;
use crate::app::models::menu::{Menu, MenuWithItem, MenuItemWithArticle};
use crate::db::schema::{ menus:: { self, dsl::* }, menu_items, articles};
use barkeel_lib::app::http::response::Response;
use diesel::prelude::*;
use std::collections::BTreeMap;

pub async fn index(State(config): State<Config>) -> impl IntoResponse  {
    let all_menus = menus::table.select(Menu::as_select()).order(menus::id.asc()).load(&mut config.database.pool.get().unwrap());
    match all_menus {
        Ok(all_menus) => {
            let menu_items_with_articles = menu_items::table
                .inner_join(articles::table)
                .select((menu_items::id, menu_items::menu_id, menu_items::article_id, menu_items::label, articles::homepage, articles::slug))
                .order(menu_items::position.asc())
                .load::<MenuItemWithArticle>(&mut config.database.pool.get().unwrap())
                .expect("Failed to load menu items");

            let mut grouped_menu_items = BTreeMap::new();
            for item in &menu_items_with_articles {
                if let Some(menu_id) = item.menu_id {
                    grouped_menu_items.entry(menu_id).or_insert_with(Vec::new);
                    grouped_menu_items.get_mut(&menu_id).unwrap().push(item);
                }
            }
            let items_per_menu: Vec<MenuWithItem> = grouped_menu_items.into_iter().map(|(menu_id, items)| {
                let menu = all_menus.iter().find(|menu| menu.id == menu_id).unwrap();
                MenuWithItem { 
                    menu: menu.clone(), 
                    items: items.to_vec() 
                }
            }).collect();

            let serialized = serde_json::to_string(&items_per_menu).unwrap();
            Response{status_code: StatusCode::OK, content_type: "application/json", datas: serialized}
        },
        _ => Response{status_code: StatusCode::OK, content_type: "application/json", datas: "".to_string()},
    }
}

pub async fn show(Path(param_id): Path<i32>, State(config): State<Config>) -> Json<String> {
    let result = menus
        .find(param_id).first::<Menu>(&mut config.database.pool.get().unwrap()) 
        .expect("Error loading data");
    let serialized = serde_json::to_string(&result).unwrap();
    Json(serialized)
}

pub async fn create(Json(payload): Json<Menu>, State(config): State<Config>) -> Json<String> {
    let inserted_record: Menu = diesel::insert_into(menus)
        .values(name.eq(payload.name))
        .get_result(&mut config.database.pool.get().unwrap())
        .expect("Error inserting data");
    let serialized = serde_json::to_string(&inserted_record).unwrap();
    Json(serialized)
}

pub async fn update(Path(param_id): Path<i32>, Json(payload): Json<Menu>, State(config): State<Config>) -> Json<String> {
    let updated_record: Menu = diesel::update(menus)
        .filter(id.eq(param_id))
        .set(name.eq(payload.name))
        .get_result(&mut config.database.pool.get().unwrap())
        .expect("Error updating data");
    let serialized = serde_json::to_string(&updated_record).unwrap();
    Json(serialized)
}

pub async fn delete(Path(param_id): Path<i32>, State(config): State<Config>) -> impl IntoResponse {
    diesel::delete(menus)
        .filter(id.eq(param_id))
        .execute(&mut config.database.pool.get().unwrap())
        .expect("Error deleting data");
    Response{status_code: StatusCode::NOT_FOUND, content_type: "application/json", datas: "Data deleted successfully".to_string()}
}
