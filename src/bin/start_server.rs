use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use juniper::RootNode;
use std::env;
use std::io;
use std::sync::Arc;

use wine_server::db::establish_pool;
use wine_server::graphql::{RootMutation, RootQuery};
use wine_server::routes::{graphiql, graphql, index, login, logout, RequestContext};

type Schema = RootNode<'static, RootQuery, RootMutation>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
  dotenv().ok();
  env::set_var("RUST_LOG", "actix_web=debug");
  // let mut listenfd = ListenFd::from_env();
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
      .wrap(IdentityService::new(
        CookieIdentityPolicy::new(&[0; 32])
          .name("wine-sessionid")
          .secure(false),
      ))
      .wrap(middleware::Logger::default())
      .service(logout)
      .service(login)
      .service(index)
      .service(graphql)
      .service(graphiql)
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
