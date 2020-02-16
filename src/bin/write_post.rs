extern crate diesel;
extern crate wine_server;

use wine_server::*;
use wine_server::db::*;
use std::io::{stdin, Read};

fn main() {
  let pool = establish_connection();
  let connection = pool.get().expect("Failed to get db connection from pool");

  println!("What would you like your title to be?");
  let mut title = String::new();
  stdin().read_line(&mut title).unwrap();
  let title = &title[..(title.len() - 1)]; // Drop the newline character
  println!(
    "\nOk! Let's write {} (Press {} when finished)\n",
    title, EOF
  );
  let mut body = String::new();
  stdin().read_to_string(&mut body).unwrap();

  let post = create_post(&connection, title.to_owned(), body);
  println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
