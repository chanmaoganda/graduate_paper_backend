use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub id: String,
    pub password: String,
}
