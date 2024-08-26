use barkeel_lib::app::Config;
use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Queryable, FormBuilder, Validate, Clone)]
#[diesel(table_name = crate::db::schema::role_permissions)]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32,

}
