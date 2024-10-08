use barkeel_lib::app::Config;
use diesel::prelude::*;
use barkeel_derives::{FormBuilder, OrderBy};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::app::models::menu::Menu;
use crate::app::models::article::Article;
use crate::db::schema::menu_items::dsl::menu_items;
#[cfg(feature = "postgres")]
use barkeel_lib::database::postgres::DB;
#[cfg(feature = "mysql")]
use barkeel_lib::database::mysql::DB;
#[cfg(feature = "sqlite")]
use barkeel_lib::database::sqlite::DB;

#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq)]
#[derive(Identifiable, Queryable, Associations, Selectable, Insertable)]
#[derive(FormBuilder, Validate, OrderBy)]
#[diesel(belongs_to(Menu))]
#[diesel(belongs_to(Article))]
#[diesel(table_name = crate::db::schema::menu_items)]
pub struct MenuItem {
    pub id: i32,
    pub menu_id: Option<i32>,
    pub article_id: Option<i32>,
    pub label: String,
    pub position: i32,
}


#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::menu_items)]
pub struct MenuItemValues {
    pub menu_id: Option<i32>,
    pub article_id: Option<i32>,
    pub label: String,
    pub position: i32,
}