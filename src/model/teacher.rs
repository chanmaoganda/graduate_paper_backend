use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::manager::RegexManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Teacher {
    pub teacher_id: String,
    pub name: String,
    pub email: Option<String>,
}

impl Teacher {
    pub fn from_row(row: Row) -> Self {
        let teacher_id = row.get(0);
        let name = row.get(1);
        let email = row.get(2);
        Self {
            teacher_id,
            name,
            email,
        }
    }

    pub fn check_valid(&self, regex: &axum::Extension<Arc<RegexManager>>) -> bool {
        if !regex.is_valid_id(&self.teacher_id) {
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
