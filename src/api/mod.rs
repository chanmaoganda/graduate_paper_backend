use axum::Router;

// mod full_paper_api;
mod login_api;
mod paper_api;
mod student_api;
mod teacher_api;

const QUERY_ID_ENDPOINT: &str = "/query";
const LIST_ENDPOINT: &str = "/list";
const REGISTER_ENDPOINT: &str = "/register";
const UNREGISTER_ENDPOINT: &str = "/unregister";

const PAPER_TABLE: &str = env!("PAPER_TABLE");
const STUDENT_TABLE: &str = env!("STUDENT_TABLE");
const TEACHER_TABLE: &str = env!("TEACHER_TABLE");

pub fn registered_apis_router() -> Router {
    let student_router = student_api::get_student_router();
    let teacher_router = teacher_api::get_teacher_router();
    let paper_router = paper_api::get_paper_router();
    // let full_paper_scope = full_paper_api::get_full_paper_apis();
    let login_router = login_api::get_login_router();

    Router::new()
        .nest("/student", student_router)
        .nest("/teacher", teacher_router)
        .nest("/paper", paper_router)
        .nest("/login", login_router)
}


#[cfg(test)]
mod api_tests {
    use reqwest::ClientBuilder;
    use serde::Serialize;
    use serde_json::Value;

    use crate::model::{Student, Teacher};

    #[tokio::test]
    async fn student_register_test() {
        let base_ip = addr();

        let valid_students = vec![
            Student {
                name: "avania".into(),
                student_id: "3022244109".into(),
                email: Some("avania@gmail.com".into()),
            },
            Student {
                name: "john".into(),
                student_id: "3022244110".into(),
                email: Some("john@gmail.com".into()),
            },
            Student {
                name: "jane".into(),
                student_id: "3022244111".into(),
                email: Some("jane@gmail.com".into()),
            },
        ];

        let invalid_students = vec![
            Student {
                name: "avania".into(),
                student_id: "3022244112".into(),
                email: Some("avania".into()),
            },
            Student {
                name: "john".into(),
                student_id: "3022244".into(),
                email: Some("john@gmail.com".into()),
            },
        ];

        let queries = valid_students
            .into_iter()
            .map(IntoQueryString::into_full_query_string)
            .map(|query| format!("{}/api/student/register?{}", base_ip, query))
            .collect::<Vec<String>>();

        let client = ClientBuilder::new().no_proxy().build().unwrap();
        for query_url in queries {
            let response = client.post(query_url).send().await.unwrap();
            assert_eq!(response.status(), 200);
        }

        let queries = invalid_students
            .into_iter()
            .map(IntoQueryString::into_full_query_string)
            .map(|query| format!("{}/api/student/register?{}", base_ip, query))
            .collect::<Vec<String>>();

        for query_url in queries {
            let response = client.post(query_url).send().await.unwrap();
            assert_eq!(response.status(), 400);
        }
    }

    #[tokio::test]
    async fn teacher_register_test() {
        let base_ip = addr();

        let valid_teachers = vec![
            Teacher {
                name: "junjie chen".into(),
                teacher_id: "0000000001".into(),
                email: Some("junjiechen@tju.edu.cn".into()),
            },
            Teacher {
                name: "zheng wang".into(),
                teacher_id: "0000000002".into(),
                email: Some("wangzheng@tju.edu.cn".into()),
            },
            Teacher {
                name: "jenifer".into(),
                teacher_id: "0000000003".into(),
                email: Some("jenifer@tju.edu.cn".into()),
            },
        ];

        let invalid_teachers = vec![
            Teacher {
                name: "invalid".into(),
                teacher_id: "0000000010".into(),
                email: Some("invalid.mail".into()),
            },
            Teacher {
                name: "invalid".into(),
                teacher_id: "00000000".into(),
                email: Some("john@tju.edu.cn".into()),
            },
        ];

        let queries = valid_teachers
            .into_iter()
            .map(IntoQueryString::into_full_query_string)
            .map(|query| format!("{}/api/teacher/register?{}", base_ip, query))
            .collect::<Vec<String>>();

        let client = ClientBuilder::new().no_proxy().build().unwrap();

        for query_url in queries {
            let response = client.post(query_url).send().await.unwrap();
            assert_eq!(response.status(), 200);
        }

        let queries = invalid_teachers
            .into_iter()
            .map(IntoQueryString::into_full_query_string)
            .map(|query| format!("{}/api/teacher/register?{}", base_ip, query))
            .collect::<Vec<String>>();

        for query_url in queries {
            let response = client.post(query_url).send().await.unwrap();
            assert_eq!(response.status(), 400);
        }
    }

    #[tokio::test]
    async fn query_test() {
        let base_ip = addr();

        let client = ClientBuilder::new().no_proxy().build().unwrap();

        let student_ids = vec!["3022244109", "3022244110", "3022244111"];

        for student_id in student_ids {
            let query_url = format!("{}/api/student/query?id={}", base_ip, student_id);
            let response = client.get(query_url).send().await.unwrap();

            assert_eq!(response.status(), 200);
        }

        let teacher_ids = vec!["0000000001", "0000000002", "0000000003"];

        for teacher_id in teacher_ids {
            let query_url = format!("{}/api/teacher/query?id={}", base_ip, teacher_id);
            let response = client.get(query_url).send().await.unwrap();

            assert_eq!(response.status(), 200);
        }
    }

    #[tokio::test]
    async fn list_test() {
        let base_ip = addr();
        let list_url = format!("{}/api/student/list", base_ip);

        let client = ClientBuilder::new().no_proxy().build().unwrap();

        let response = client.get(list_url).send().await.unwrap();
        assert_eq!(response.status(), 200);

        let list_url = format!("{}/api/teacher/list", base_ip);
        let response = client.get(list_url).send().await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn login_test() {
        let base_ip = addr();
        let student_url = format!("{}/api/login/student?id={}", base_ip, "3022244109");
        let teacher_url = format!("{}/api/login/teacher?id={}", base_ip, "0000000001");

        let client = ClientBuilder::new().no_proxy().build().unwrap();

        let response = client.get(student_url).send().await.unwrap();
        assert_eq!(response.status(), 200);

        let response = client.get(teacher_url).send().await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn student_unregister_test() {
        let base_ip = addr();

        let valid_students = vec![
            Student {
                name: "avania".into(),
                student_id: "3022244109".into(),
                email: Some("avania@gmail.com".into()),
            },
            Student {
                name: "john".into(),
                student_id: "3022244110".into(),
                email: Some("john@gmail.com".into()),
            },
            Student {
                name: "jane".into(),
                student_id: "3022244111".into(),
                email: Some("jane@gmail.com".into()),
            },
        ];

        let queries = valid_students
            .into_iter()
            .map(IntoQueryString::into_id_name_query_string)
            .map(|query| format!("{}/api/student/unregister?{}", base_ip, query))
            .collect::<Vec<String>>();

        let client = ClientBuilder::new().no_proxy().build().unwrap();

        for query_url in queries {
            dbg!(&query_url);
            let response = client.post(query_url).send().await.unwrap();
            assert_eq!(response.status(), 200);
        }
    }

    #[tokio::test]
    async fn teacher_unregister_test() {
        let base_ip = addr();

        let valid_teachers = vec![
            Teacher {
                name: "junjie chen".into(),
                teacher_id: "0000000001".into(),
                email: Some("junjiechen@tju.edu.cn".into()),
            },
            Teacher {
                name: "zheng wang".into(),
                teacher_id: "0000000002".into(),
                email: Some("wangzheng@tju.edu.cn".into()),
            },
            Teacher {
                name: "jenifer".into(),
                teacher_id: "0000000003".into(),
                email: Some("jenifer@tju.edu.cn".into()),
            },
        ];

        let queries = valid_teachers
            .into_iter()
            .map(IntoQueryString::into_id_name_query_string)
            .map(|query| format!("{}/api/teacher/unregister?{}", base_ip, query))
            .collect::<Vec<String>>();

        let client = ClientBuilder::new().no_proxy().build().unwrap();

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

    pub trait IntoQueryString: Serialize + Sized {
        fn into_full_query_string(self) -> String;

        fn into_id_name_query_string(self) -> String;
    }

    impl<T: Serialize> IntoQueryString for T {
        fn into_full_query_string(self) -> String {
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
                        }
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

        fn into_id_name_query_string(self) -> String {
            let serialized = serde_json::to_value(self).unwrap();

            let mut query_pairs = Vec::new();
            if let Value::Object(values) = serialized {
                for (key, value) in values {
                    match value {
                        Value::String(s) => {
                            if key.contains("id") || key.contains("name") {
                                query_pairs.push(format!("{}={}", key, s));
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
            query_pairs.join("&")
        }
    }
}