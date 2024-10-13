use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod test {
    use actix_web::{body::MessageBody, test};

    use super::*;

    #[actix_web::test]
    async fn test_health_check() {
        let req = test::TestRequest::default().to_http_request();
        let response = health_check().await;
        let response = response.respond_to(&req);

        assert!(response.status().is_success());
        let body = response.into_body().try_into_bytes();
        match body {
            Ok(body) => assert!(body.is_empty()),
            Err(_) => panic!("Failed to convert body into bytes"),
        }
    }
}
