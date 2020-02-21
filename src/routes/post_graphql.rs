use actix_web::{post, web, Error, HttpResponse};
use std::sync::Arc;
use juniper::http::GraphQLRequest;
use uuid::Uuid;

use super::RequestContext;
use crate::graphql::GraphqlContext;


#[post("/graphql")]
pub async fn graphql(
  context: web::Data<Arc<RequestContext>>,
  data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {

  let graphql_result = web::block(move || {
    let graphql_context = GraphqlContext {
      db_pool: context.db_pool.clone(),
      user_id: Uuid::new_v4(),
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