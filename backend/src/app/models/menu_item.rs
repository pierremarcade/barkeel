use crate::config::application::Config;
use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::app::models::menu::Menu;
use crate::app::models::article::Article;
use crate::db::schema::menu_items;

#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq)]
#[derive(Identifiable, Queryable, Associations, Selectable, Insertable)]
#[derive(FormBuilder, Validate)]
#[diesel(belongs_to(Menu))]
#[diesel(belongs_to(Article))]
#[diesel(table_name = menu_items)]
pub struct MenuItem {
    pub id: i32,
    pub menu_id: Option<i32>,
    pub article_id: Option<i32>,
    #[validate(length(min = 4))]
    pub label: String,
    pub position: i32,
}


#[derive(Insertable, AsChangeset)]
#[diesel(table_name = menu_items)]
pub struct MenuItemValues {
    pub menu_id: Option<i32>,
    pub article_id: Option<i32>,
    pub label: String,
    pub position: i32,
}