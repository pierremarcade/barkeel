use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FormBuilder, Clone)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}