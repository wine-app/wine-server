use actix_web::{get, HttpResponse, http};
use juniper::http::playground::playground_source;
use actix_identity::Identity;


#[get("/graphiql")]
pub async fn graphiql(
  id: Identity,
) -> HttpResponse {
  // Redirect on no identity
  if let None = id.identity() {
    return HttpResponse::Found()
      .header(http::header::LOCATION, "/login_page")
      .finish();
  }
  let html = playground_source("http://localhost:8080/graphql");
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}