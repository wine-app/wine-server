extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate serde_json;
extern crate wine_server;

use wine_server::db::PgPool;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use juniper::RootNode;
use std::io;
use std::sync::Arc;
use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use uuid::Uuid;

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
  id: Identity,
) -> Result<HttpResponse, Error> {
  // if let None = id.identity() {
  //   return Ok(HttpResponse::Unauthorized().finish());
  // }

  // if let Some(user_id) = id.identity() {
  //   // check id against service
  //   // get facebook or google user id
  // }

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

async fn index() -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("hello there"),
  )
}

struct RequestContext {
  schema: Schema,
  db_pool: Arc<PgPool>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
  dotenv().ok();
  std::env::set_var("RUST_LOG", "actix_web=debug");
  env_logger::init();

  let db_pool = establish_pool();

  // Create Juniper schema
  let schema = Schema::new(RootQuery, RootMutation);
  let context = Arc::new(RequestContext {
    schema,
    db_pool: Arc::new(db_pool),
  });

  // Start http server
  HttpServer::new(move || {
    App::new()
      .data(context.clone())
      // .wrap(IdentityService::new(
      //   CookieIdentityPolicy::new(&[0; 32])
      //       .name("wine-sessionid")
      //       .secure(false),
      // ))
      .wrap(middleware::Logger::default())
      .service(web::resource("/").route(web::get().to(index)))
      .service(web::resource("/graphql").route(web::post().to(graphql)))
      .service(web::resource("/graphiql").route(web::get().to(graphiql)))
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
