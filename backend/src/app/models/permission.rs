use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Queryable, FormBuilder, Clone)]
#[diesel(table_name = crate::db::schema::permissions)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,

}
