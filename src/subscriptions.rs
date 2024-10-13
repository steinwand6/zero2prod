use actix_web::{web::Form, HttpResponse, Responder};

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn subscribe_with_valid_input() {
        let form_data = FormData {
            email: "test@example.com".to_string(),
            name: "test".to_string(),
        };
        let req = test::TestRequest::default().to_http_request();
        let input = Form(form_data);
        let response = subscribe(input).await;
        let response = response.respond_to(&req);
        assert!(response.status().is_success());
    }
}
