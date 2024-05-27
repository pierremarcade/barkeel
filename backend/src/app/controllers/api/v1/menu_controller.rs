use axum::{ extract::{Path, State}, response::{ Json, IntoResponse }, http::StatusCode };
use crate::config::application::Config;
use crate::app::models::menu::Menu;
use crate::app::models::menu_item::MenuItem;
use crate::app::models::article::ArticleMenu;
use serde::Serialize;
use crate::db::schema::{ menus, menu_items, articles};
use crate::db::schema::menus::dsl::*;
use crate::app::utils::{ response::Response };
use diesel::prelude::*;
use std::sync::Arc;

#[derive(Serialize)]
struct MenuWithItem {
    #[serde(flatten)]
    menu: Menu,
    items: Vec<(MenuItem, ArticleMenu)>,
}

pub async fn index(State(config): State<Arc<Config>>) -> impl IntoResponse  {
    let all_menus = menus::table.select(Menu::as_select()).load(&mut config.database.pool.get().unwrap());
    match all_menus {
        Ok(all_menus) => {

            let menu_items = menu_items::table
            .inner_join(articles::table)
            .select((MenuItem::as_select(), ArticleMenu::as_select()))
            .load::<(MenuItem, ArticleMenu)>(&mut config.database.pool.get().unwrap());

            let items_per_menu = menu_items.expect("REASON")
                .grouped_by(&all_menus)
                .into_iter()
                .zip(all_menus)
                .map(|(items, menu)| MenuWithItem {menu, items})
                .collect::<Vec<MenuWithItem>>();

            let serialized = serde_json::to_string(&items_per_menu).unwrap();
            return Response{status_code: StatusCode::OK, content_type: "application/json", datas: serialized};
        },
        _ => Response{status_code: StatusCode::OK, content_type: "application/json", datas: "".to_string()},
    }
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
        .values(name.eq(payload.name))
        .get_result(&mut config.database.pool.get().unwrap())
        .expect("Error inserting data");
    let serialized = serde_json::to_string(&inserted_record).unwrap();
    Json(serialized)
}

pub async fn update(Path(param_id): Path<i32>, Json(payload): Json<Menu>, State(config): State<Arc<Config>>) -> Json<String> {
    let updated_record: Menu = diesel::update(menus)
        .filter(id.eq(param_id))
        .set(name.eq(payload.name))
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


