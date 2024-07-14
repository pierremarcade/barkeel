use crate::config::application::Config;
use serde::{Deserialize, Serialize};
use barkeel_derives::FormBuilder;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Clone, FormBuilder)]
#[form_builder(configuration(action_button="Login"))]
pub struct Credentials {
    pub email: String,
    pub password: String,
}