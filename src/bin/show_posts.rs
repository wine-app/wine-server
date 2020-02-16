extern crate diesel;
extern crate wine_server;

use self::diesel::prelude::*;
use wine_server::*;
use wine_server::db::*;
use self::models::*;

fn main() {
  use wine_server::schema::posts::dsl::*;

  let pool = establish_connection();
  let connection = pool.get().expect("Failed to get db connection from pool");
  let results = posts
    .filter(published.eq(true))
    .limit(5)
    .load::<Post>(&connection)
    .expect("Error loading posts");

  println!("Displaying {} posts", results.len());
  for post in results {
    println!("{}", post.title);
    println!("----------\n");
    println!("{}", post.body);
  }
}
