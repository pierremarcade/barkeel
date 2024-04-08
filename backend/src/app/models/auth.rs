use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}