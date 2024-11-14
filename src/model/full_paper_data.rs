use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct FullPaperData {
    pub paper_base_id: i32,
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
        let paper_base_id: i32 = row.get(0);
        let student_name: String = row.get(1);
        let student_id: String = row.get(2);
        let student_email: Option<String> = row.get(3);
        let title: String = row.get(4);
        let teacher_name: String = row.get(5);
        let teacher_id: String = row.get(6);
        let teacher_email: Option<String> = row.get(7);

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
