use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct Paper {
    pub id: i32,
    pub student_id: String,
    pub title: String,
    // TODO: status???
}

impl Paper {
    pub fn from_row(row: Row) -> Self {
        let id: i32 = row.get(0);
        let student_id: String = row.get(1);
        let title: String = row.get(2);
        Self { id, student_id, title }
    }
}
