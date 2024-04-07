use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Queryable, FormBuilder, Clone)]
#[diesel(table_name = crate::db::schema::menu_items)]
pub struct MenuItem {
    pub id: i32,
    pub menu_id: Option<i32>,
    pub label: String,
    pub link: String,
    pub position: i32,

}
