mod full_paper_api;
mod paper_api;
mod student_api;
mod teacher_api;
mod login_api;

use actix_web::web;
use serde::Serialize;
use serde_json::Value;

const QUERY_ID_ENDPOINT: &str = "/query";
const LIST_ENDPOINT: &str = "/list";
const REGISTER_ENDPOINT: &str = "/register";

const PAPER_TABLE: &str = env!("PAPER_TABLE");
const STUDENT_TABLE: &str = env!("STUDENT_TABLE");
const TEACHER_TABLE: &str = env!("TEACHER_TABLE");

pub fn configure_apis(cfg: &mut web::ServiceConfig) {
    let student_scope = student_api::get_student_apis();
    let teacher_scope = teacher_api::get_teacher_apis();
    let paper_scope = paper_api::get_paper_apis();
    let full_paper_scope = full_paper_api::get_full_paper_apis();
    let login_service = login_api::get_login_api();

    let api_scope = web::scope("/api")
        .service(student_scope)
        .service(teacher_scope)
        .service(paper_scope)
        .service(full_paper_scope);

    cfg.service(api_scope)
       .service(login_service);
}

pub trait IntoQueryString: Serialize + Sized {
    fn into_query_string(self) -> String ;
}

impl<T: Serialize> IntoQueryString for T {
    fn into_query_string(self) -> String {
        let serialized = serde_json::to_value(self).unwrap();

        let mut query_pairs = Vec::new();
        if let Value::Object(values) = serialized {
            for (key, value) in values {
                let result = match value {
                    Value::String(s) => {
                        format!("{}={}", key, s)
                    }
                    Value::Number(n) => {
                        if let Some(num) = n.as_u64() {
                            format!("{key}={num}")
                        } else {
                            panic!("Number not invalid")
                        }
                    }
                    Value::Bool(b) => {
                        format!("{}={}", key, b)
                    },
                    _ => {
                        log::debug!("Unsupported type");
                        continue;
                    } // Handle other types if necessary
                };
                query_pairs.push(result)
            }
        }
        query_pairs.join("&")
    }
}

#[cfg(test)]
mod api_tests {
    use crate::{api::IntoQueryString, model::Student};


    #[tokio::test]
    async fn query_test() {
        let base_ip = addr();
        let query_url = format!("{}student/query?id={}", base_ip, "3022244109");
        let response = reqwest::get(query_url).await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn list_test() {
        let base_ip = addr();
        let list_url = format!("{}/api/student/list", base_ip);
        
        let response = reqwest::get(list_url).await.unwrap();
        assert_eq!(response.status(), 200);

        let list_url = format!("{}/api/teacher/list", base_ip);
        let response = reqwest::get(list_url).await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn login_test() {
        let base_ip = addr();
        let student_url = format!("{}/login/student?id={}", base_ip, "3022244109");
        let teacher_url = format!("{}/login/teacher?id={}", base_ip, "1111111111");

        let response = reqwest::get(student_url).await.unwrap();
        assert_eq!(response.status(), 200);

        let response = reqwest::get(teacher_url).await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn student_register_test() {
        let base_ip = addr();

        let students = vec![
            Student { name: "avania".into(), student_id: "3022244109".into(), email: Some("avania@gmail.com".into()) },
            Student { name: "john".into(), student_id: "3022244110".into(), email: Some("john@gmail.com".into()) },
            Student { name: "jane".into(), student_id: "3022244111".into(), email: Some("jane@gmail.com".into()) },
        ];

        let queries = students
            .into_iter()
            .map(IntoQueryString::into_query_string)
            .map(|query| format!("{}/api/student/register?{}", base_ip, query))
            .collect::<Vec<String>>();

        let client = reqwest::Client::new();
        for query_url in queries {
            let response = client.post(query_url).send().await.unwrap();
            assert_eq!(response.status(), 200);
        }
    }

    fn addr() -> String {
        let address = env!("ADDRESS");
        let port = env!("PORT");
        format!("http://{}:{}", address, port)
    }
}