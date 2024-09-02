use barkeel_lib::app::Config;
use diesel::prelude::*;
use barkeel_derives::{FormBuilder, OrderBy};
use serde::{Deserialize, Serialize};
use crate::db::schema::menus::dsl::menus;
use validator::Validate;
#[cfg(feature = "postgres")]
use barkeel_lib::database::postgres::DB;
#[cfg(feature = "mysql")]
use barkeel_lib::database::mysql::DB;
#[cfg(feature = "sqlite")]
use barkeel_lib::database::sqlite::DB;

#[derive(Serialize, Deserialize, Identifiable, Queryable, Selectable, FormBuilder, Validate, Clone, OrderBy)]
#[diesel(table_name = crate::db::schema::menus)]
#[form_builder(label = name, id = id)]
pub struct Menu {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize)]
pub struct MenuWithItem<'a> {
    #[serde(flatten)]
    pub menu: Menu,
    pub items: Vec<&'a MenuItemWithArticle>,
}

#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct MenuItemWithArticle{
    pub id: i32,
    pub menu_id: Option<i32>,
    pub article_id: Option<i32>,
    pub label: String,
    pub homepage: bool,
    pub slug: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::menus)]
pub struct MenuValues {
    pub name: String,
}
