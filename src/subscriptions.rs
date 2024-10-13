use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> impl Responder {
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
      VALUES ($1, $2, $3, $4);
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Failed to execute query {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

/*
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
*/
