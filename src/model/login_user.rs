use serde::{Deserialize, Serialize};

use super::{Student, Teacher};

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub id: String,
    pub password: String,
}

impl From<Student> for LoginUser {
    fn from(student: Student) -> Self {
        Self {
            id: student.id,
            password: student.password,
        }
    }
}

impl From<Teacher> for LoginUser {
    fn from(teacher: Teacher) -> Self {
        Self {
            id: teacher.id, 
            password: teacher.password,
        }
    }
}