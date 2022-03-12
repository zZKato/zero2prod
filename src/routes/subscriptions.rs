use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
  email: String,
  name: String,
}

#[tracing::instrument(
  name = "Adding a new subscriber",
  skip(form, conn_pool),
  fields(
    subscriber_email = %form.email,
    subscriber_name = %form.name
  )
)]
pub async fn subscribe(form: web::Form<FormData>, conn_pool: web::Data<PgPool>) -> HttpResponse {
  match insert_subscriber(&conn_pool, &form).await
  {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().finish()
  }
}

#[tracing::instrument(
  name = "Saving new subscriber details in the database",
  skip(form, conn_pool)
)]
pub async fn insert_subscriber(conn_pool: &web::Data<PgPool>, form: &web::Form<FormData>) -> Result<(), sqlx::Error> {
  sqlx::query!(
    r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now()
  )
  .execute(conn_pool.get_ref())
  .await
  .map_err(|e| {
    tracing::error!("Failed to execute query: {:?}", e);
    e
  })?;
  Ok(())
}
