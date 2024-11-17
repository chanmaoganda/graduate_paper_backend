use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::manager::RegexManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub student_id: String,
    pub email: Option<String>,
}

impl Student {
    pub fn from_row(row: Row) -> Self {
        let name = row.get(0);
        let student_id = row.get(1);
        let email = row.get(2);
        Self {
            name,
            student_id,
            email,
        }
    }

    pub fn check_valid(&self, regex: &axum::Extension<Arc<RegexManager>>) -> bool {
        log::debug!("Checking if student ({}, {}) is valid", self.name, self.student_id);

        if !regex.is_valid_id(&self.student_id) {
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
