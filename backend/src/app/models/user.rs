use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Selectable, Identifiable, PartialEq, Queryable, FormBuilder, Clone)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role_id: Option<i32>,

}
