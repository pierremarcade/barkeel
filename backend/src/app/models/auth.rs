use crate::config::application::Config;
use serde::{Deserialize, Serialize};
use barkeel_derives::FormBuilder;

#[derive(Serialize, Deserialize, Clone, FormBuilder)]
#[form_builder(configuration(action_button="Login"))]
pub struct Credentials {
    pub email: String,
    pub password: String,
}