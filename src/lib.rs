use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

#[cfg(test)]
mod test {
    use actix_web::{body::MessageBody, test};

    use super::*;

    #[actix_web::test]
    async fn test_health_check() {
        let app =
            test::init_service(App::new().route("/health_check", web::get().to(health_check)))
                .await;
        let req = test::TestRequest::get().uri("/health_check").to_request();
        let res = test::call_service(&app, req).await;

        let status = res.status();
        let body = res.into_body().try_into_bytes().unwrap();

        assert_eq!(status, 200, "status isn't 200.");
        assert!(body.is_empty(), "body is not empty.");
    }
}
