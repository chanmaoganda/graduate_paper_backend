use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullPaperData {
    pub paper_base_id: Option<i32>,
    pub student_name: String,
    pub student_id: String,
    pub student_email: Option<String>,
    pub title: String,
    pub teacher_name: String,
    pub teacher_id: String,
    pub teacher_email: Option<String>,
}

impl FullPaperData {
    pub fn from_row(row: Row) -> Self {
        let paper_base_id = Some(row.get(0));
        let student_name = row.get(1);
        let student_id = row.get(2);
        let student_email = row.get(3);
        let title = row.get(4);
        let teacher_name = row.get(5);
        let teacher_id = row.get(6);
        let teacher_email = row.get(7);

        FullPaperData {
            paper_base_id,
            student_name,
            student_id,
            student_email,
            title,
            teacher_name,
            teacher_id,
            teacher_email,
        }
    }
}
