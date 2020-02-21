use actix_web::{get, Error, HttpResponse};

#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("hello there"),
  )
}