use diesel_derive_enum::DbEnum;

use crate::schema::*;

#[derive(DbEnum, PartialEq, Debug)]
#[DieselType = "Wine_color"]
pub enum WineColor {
  #[db_rename = "white"]
  White,
  #[db_rename = "red"]
  Red,
}