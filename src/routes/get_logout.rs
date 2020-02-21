use actix_identity::Identity;
use actix_web::{get, Error, HttpResponse};

#[get("/logout")]
pub async fn logout(
  id: Identity,
) -> Result<HttpResponse, Error> {
  id.forget();
  Ok(HttpResponse::Ok().into())
}