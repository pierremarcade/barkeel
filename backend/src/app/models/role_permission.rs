use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Queryable, FormBuilder, Clone)]
#[diesel(table_name = crate::db::schema::role_permissions)]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32,

}
