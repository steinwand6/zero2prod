use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[instrument(name = "Adding a new subscriber.",
            skip(form, pool),
            fields(request_id = %Uuid::new_v4(), subscriber_email = %form.email, subscriber_name = %form.name))]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    match insert_subscriber(form, pool).await {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved",);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[instrument(name = "Saving new subscriber details in the database",
            skip(form, pool),
            fields(subscriber_email = %form.email,subscriber_name = %form.name))]
pub async fn insert_subscriber(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
      VALUES ($1, $2, $3, $4);
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {e}");
        e
    })?;
    Ok(())
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
