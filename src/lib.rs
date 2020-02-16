#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub mod schema;
pub mod models;
pub mod graphql_resolvers;
pub mod db;

use self::models::{Post, NewPost};

pub fn create_post<'a>(conn: &PgConnection, title: String, body: String) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}