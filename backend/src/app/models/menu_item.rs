use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};
use crate::app::models::menu::Menu;
use crate::db::schema::menu_items;

#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq)]
#[derive(Identifiable, Queryable, Associations, Selectable)]
#[derive(FormBuilder)]
#[diesel(belongs_to(Menu))]
#[diesel(table_name = menu_items)]
pub struct MenuItem {
    pub id: i32,
    pub menu_id: Option<i32>,
    pub label: String,
    pub link: String,
    pub position: i32,
}
