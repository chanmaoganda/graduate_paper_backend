use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
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
}
