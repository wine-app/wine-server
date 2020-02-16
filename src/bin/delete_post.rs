extern crate diesel;
extern crate wine_server;

use self::diesel::prelude::*;
use wine_server::db::*;
use std::env::args;

fn main() {
  use wine_server::schema::posts::dsl::*;

  let target = args().nth(1).expect("Expected a target to match against");
  let pattern = format!("%{}%", target);

  let pool = establish_connection();
  let connection = pool.get().expect("Failed to get db connection from pool");
  let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
    .execute(&connection)
    .expect("Error deleting posts");

  println!("Deleted {} posts", num_deleted);
}
