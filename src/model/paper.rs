use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    pub base_id: Option<i32>,
    pub student_id: String,
    pub teacher_id: String,
    pub title: String,
    // TODO: status???
}

impl Paper {
    pub fn from_row(row: Row) -> Self {
        let base_id = Some(row.get("base_id"));
        let student_id = row.get("student_id");
        let teacher_id = row.get("teacher_id");
        let title = row.get("title");
        Self {
            base_id,
            student_id,
            teacher_id,
            title,
        }
    }
}
