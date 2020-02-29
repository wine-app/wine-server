use actix_web::{post, web, Error, HttpResponse, http};
use std::sync::Arc;
use juniper::http::GraphQLRequest;
use serde_derive::Serialize;
use actix_identity::Identity;

use super::RequestContext;
use crate::graphql::GraphqlContext;

#[post("/graphql")]
pub async fn graphql(
  context: web::Data<Arc<RequestContext>>,
  id: Identity,
  data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {

  if let None = id.identity() {
    return Ok(HttpResponse::Found()
      .header(http::header::LOCATION, "/login_page")
      .finish());
  }

  let graphql_result = web::block(move || {
    let graphql_context = GraphqlContext {
      db_pool: context.db_pool.clone(),
      user_id: 0,
    };
    let res = data.execute(&context.schema, &graphql_context);
    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
  })
  .await?;
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(graphql_result),
  )
}