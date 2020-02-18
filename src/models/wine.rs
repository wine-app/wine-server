use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

use crate::schema::*;

#[derive(DbEnum, PartialEq, Debug)]
#[DieselType = "Wine_intensity"]
pub enum WineIntensity {
  #[db_rename = "delicate"]
  Delicate,
  #[db_rename = "moderate"]
  Moderate,
  #[db_rename = "powerful"]
  Powerful,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "wines"]
#[primary_key(name, producer, vintage)]
pub struct Wine {
  pub id: i32,
  pub name: String,
  pub producer: String,
  pub vintage: i32,
  pub region: String,
  pub country: String,
  pub sparkling: bool,
  pub sweetness: i32,
  pub tannin: i32,
  pub acid: i32,
  pub alcohol: i32,
  pub body: i32,
  pub intensity: WineIntensity,
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "wines"]
#[primary_key(name, producer, vintage)]
pub struct WineInput {
  pub name: String,
  pub producer: String,
  pub vintage: i32,
  pub region: String,
  pub country: String,
  pub sparkling: bool,
  pub sweetness: i32,
  pub tannin: i32,
  pub acid: i32,
  pub alcohol: i32,
  pub body: i32,
  pub intensity: WineIntensity,
}

#[derive(Associations, AsChangeset, Insertable, Queryable, Debug, Clone)]
#[belongs_to(parent = "Wine")]
#[table_name = "compositions"]
pub struct Composition {
  pub wine_id: i32,
  pub grape: String,
  pub percent: i32,
}
