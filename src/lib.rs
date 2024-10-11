use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, Form},
    App, HttpResponse, HttpServer, Responder,
};

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
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
