#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate juniper;
#[macro_use] extern crate log;

extern crate ndarray;
extern crate uuid;
extern crate diesel_derive_enum;
extern crate dotenv;

pub mod db;
pub mod models;
pub mod graphql;
pub mod schema;
pub mod routes;
pub mod algorithm;

pub fn hello_world() -> &'static str {
  "hello world"
}

use std::fmt::Display;

#[derive(Debug)]
struct PrependedDisplay<'a, E: Display> (E, &'a str);

impl<T: Display> Display for PrependedDisplay<'_, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}: {}", self.1, self.0)
  }
}

trait PrependError<T, E: Display>{
  fn prepend_err(self, msg: &str) -> Result<T, PrependedDisplay<E>>;
}

impl<T, E: Display> PrependError<T, E> for Result<T, E> {
  fn prepend_err(self, msg: &str) -> Result<T, PrependedDisplay<E>> {
    match self {
      Ok(val) => Ok(val),
      Err(e) => Err(PrependedDisplay(e, msg)),
    }
  }
}