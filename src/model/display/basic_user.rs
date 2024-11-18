use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct BasicUser {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
}

impl BasicUser {
    pub fn from_row(row: Row) -> Self {
        let id = row.get("id");
        let name = row.get("name");
        let email = row.get("email");
        Self {
            id,
            name,
            email,
        }
    }
}