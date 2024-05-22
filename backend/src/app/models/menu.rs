use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Queryable, FormBuilder, Clone)]
#[diesel(table_name = crate::db::schema::menus)]
#[form_builder(label = name, id = id)]
pub struct Menu {
    pub id: i32,
    pub name: String,
}
