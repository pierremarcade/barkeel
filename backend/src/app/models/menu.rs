use barkeel_lib::app::Config;
use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};
use crate::db::schema::menus;
use validator::Validate;

#[derive(Serialize, Deserialize, Identifiable, Queryable, Selectable, FormBuilder, Validate, Clone)]
#[diesel(table_name = menus)]
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
#[diesel(table_name = menus)]
pub struct MenuValues {
    pub name: String,
}
