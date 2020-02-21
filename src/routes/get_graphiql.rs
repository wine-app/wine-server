use actix_web::{get, HttpResponse};
use juniper::http::playground::playground_source;

#[get("/graphiql")]
pub async fn graphiql() -> HttpResponse {
  let html = playground_source("http://localhost:8080/graphql");
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}