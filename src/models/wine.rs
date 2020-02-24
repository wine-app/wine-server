use crate::schema::*;

use super::WineColor;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "wines"]
#[primary_key(name, producer, vintage)]
pub struct Wine {
  pub id: i32,
  pub name: String,
  pub color: WineColor,
  pub producer: String,
  pub vintage: i32,
  pub region: String,
  pub country: String,
  pub sparkling: bool,
  pub alcohol: f64,
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "wines"]
#[primary_key(name, producer, vintage)]
pub struct WineInput {
  pub name: String,
  pub color: WineColor,
  pub producer: String,
  pub vintage: i32,
  pub region: String,
  pub country: String,
  pub sparkling: bool,
  pub alcohol: f64,
}