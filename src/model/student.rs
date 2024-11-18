use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::manager::RegexManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub password: String,
    pub email: Option<String>,
}

impl Student {
    #[allow(dead_code)]
    pub fn from_row(row: Row) -> Self {
        let id = row.get("id");
        let name = row.get("name");
        let password = row.get("password");
        let email = row.get("email");
        Self {
            id,
            name,
            password,
            email,
        }
    }

    pub fn check_valid(&self, regex: &axum::Extension<Arc<RegexManager>>) -> bool {
        if !regex.is_valid_id(&self.id) {
            return false;
        }
        if let Some(email) = &self.email {
            if !regex.is_valid_email(email) {
                return false;
            }
        }
        true
    }
}
