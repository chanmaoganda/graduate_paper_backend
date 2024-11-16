mod full_paper_api;
mod paper_api;
mod student_api;
mod teacher_api;
mod login_api;

use actix_web::web;

const QUERY_ID_ENDPOINT: &str = "/query/{id}";
const LIST_ENDPOINT: &str = "/list";

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

#[cfg(test)]
mod api_tests {

    #[tokio::test]
    async fn login_test() {
        let base_ip = addr();
        let student_url = format!("{}/login/student?id={}", base_ip, "3022244109");
        let teacher_url = format!("{}/login/teacher?id={}", base_ip, "1111111111");

        dbg!(&student_url);
        dbg!(&teacher_url);

        let response = reqwest::get(student_url).await.unwrap();
        assert_eq!(response.status(), 200);
        assert_eq!(response.text().await.unwrap(), "student 3022244109 login successful");

        let response = reqwest::get(teacher_url).await.unwrap();
        assert_eq!(response.status(), 200);
        assert_eq!(response.text().await.unwrap(), "teacher 1111111111 login successful");
    }

    

    fn addr() -> String {
        let address = env!("ADDRESS");
        let port = env!("PORT");
        format!("http://{}:{}", address, port)
    }
}