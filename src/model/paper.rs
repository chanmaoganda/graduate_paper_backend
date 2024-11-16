use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct Paper {
    pub base_id: Option<i32>,
    pub student_id: String,
    pub teacher_id: String,
    pub title: String,
    // TODO: status???
}

impl Paper {
    pub fn from_row(row: Row) -> Self {
        let base_id = Some(row.get(0));
        let student_id = row.get(1);
        let teacher_id = row.get(2);
        let title = row.get(3);
        Self {
            base_id,
            student_id,
            teacher_id,
            title,
        }
    }
}
