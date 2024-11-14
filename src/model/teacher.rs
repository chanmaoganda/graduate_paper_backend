use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct Teacher {
    base_id: i32,
    name: String,
    teacher_id: String,
    email: Option<String>,
}

impl Teacher {
    pub fn from_row(row: Row) -> Self {
        let base_id: i32 = row.get(0);
        let name: String = row.get(1);
        let teacher_id: String = row.get(2);
        let email: Option<String> = row.get(3);
        Self {
            base_id,
            name,
            teacher_id,
            email,
        }
    }
}
