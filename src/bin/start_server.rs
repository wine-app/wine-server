extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate serde_json;
extern crate wine_server;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use juniper::RootNode;
use std::io;
use std::sync::Arc;

use wine_server::db::establish_pool;
use wine_server::graphql::{GraphqlContext, RootQuery, RootMutation};

type Schema = RootNode<'static, RootQuery, RootMutation>;

async fn graphiql() -> HttpResponse {
  let html = playground_source("http://localhost:8080/graphql");
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}

async fn graphql(
  context: web::Data<Arc<RequestContext>>,
  data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
  let user = web::block(move || {
    let res = data.execute(&context.schema, &context.graphql_context);
    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
  })
  .await?;
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(user),
  )
}

struct RequestContext {
  schema: Schema,
  graphql_context: GraphqlContext,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
  dotenv().ok();
  std::env::set_var("RUST_LOG", "actix_web=debug");
  env_logger::init();

  let db_pool = establish_pool();

  // Create Juniper schema
  let schema = Schema::new(RootQuery, RootMutation);
  let graphql_context = GraphqlContext { db_pool };
  let context = Arc::new(RequestContext {
    schema,
    graphql_context,
  });

  // Start http server
  HttpServer::new(move || {
    App::new()
      .data(context.clone())
      .wrap(middleware::Logger::default())
      .service(web::resource("/graphql").route(web::post().to(graphql)))
      .service(web::resource("/graphiql").route(web::get().to(graphiql)))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
