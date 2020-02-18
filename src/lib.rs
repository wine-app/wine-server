#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate juniper;
#[macro_use] extern crate log;

extern crate diesel_derive_enum;
extern crate dotenv;

pub mod db;
pub mod models;
pub mod graphql;
pub mod schema;

pub fn hello_world() -> &'static str {
  "hello world"
}

// use self::models::{NewPost, Post};

// pub fn create_post<'a>(conn: &PgConnection, title: String, body: String) -> Post {
//   use schema::posts;

//   let new_post = NewPost {
//     title: title,
//     body: body,
//   };

//   diesel::insert_into(posts::table)
//     .values(&new_post)
//     .get_result(conn)
//     .expect("Error saving new post")
// }
