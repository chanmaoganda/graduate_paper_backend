use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub base_id: i32,
    pub name: String,
    pub student_id: String,
    pub email: Option<String>,
}

impl Student {
    pub fn from_row(row: Row) -> Self {
        let base_id: i32 = row.get(0);
        let name: String = row.get(1);
        let student_id: String = row.get(2);
        let email: Option<String> = row.get(3);
        Self {
            base_id,
            name,
            student_id,
            email,
        }
    }
}
