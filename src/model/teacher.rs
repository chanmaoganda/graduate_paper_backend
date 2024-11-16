use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct Teacher {
    pub name: String,
    pub teacher_id: String,
    pub email: Option<String>,
}

impl Teacher {
    pub fn from_row(row: Row) -> Self {
        let name = row.get(0);
        let teacher_id = row.get(1);
        let email = row.get(2);
        Self {
            name,
            teacher_id,
            email,
        }
    }
}
