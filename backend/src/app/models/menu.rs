use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};
use crate::db::schema::menus;


#[derive(Serialize, Deserialize, Identifiable, Queryable, Selectable, FormBuilder, Clone)]
#[diesel(table_name = menus)]
#[form_builder(label = name, id = id)]
pub struct Menu {
    pub id: i32,
    pub name: String,
}