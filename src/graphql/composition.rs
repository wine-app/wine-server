use diesel::prelude::*;
use juniper::FieldResult;

use crate::models::{Composition as DbComposition};

#[derive(Debug, juniper::GraphQLObject, Clone)]
pub struct Composition {
  pub grape: String,
  pub percent: i32,
}

impl From<DbComposition> for Composition {
  fn from(item: DbComposition) -> Composition {
    Composition {
      grape: item.grape,
      percent: item.percent,
    }
  }
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
pub struct CompositionInput {
  pub grape: String,
  pub percent: i32,
}