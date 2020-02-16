extern crate diesel;
extern crate wine_server;

use self::diesel::prelude::*;
use wine_server::db::*;
use wine_server::models::Post;
use std::env::args;

fn main() {
  use wine_server::schema::posts::dsl::{posts, published};

  let id = args()
    .nth(1)
    .expect("publish_post requires a post id")
    .parse::<i32>()
    .expect("Invalid ID");
  let pool = establish_connection();
  let connection = pool.get().expect("Failed to get db connection from pool");

  let post = diesel::update(posts.find(id))
    .set(published.eq(true))
    .get_result::<Post>(&connection)
    .expect(&format!("Unable to find post {}", id));
  println!("Published post {}", post.title);
}
