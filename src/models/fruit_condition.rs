use diesel_derive_enum::DbEnum;

use crate::schema::*;

#[derive(DbEnum, PartialEq, Debug)]
#[DieselType = "Fruit_condition"]
pub enum FruitCondition {
  #[db_rename = "tart"]
  Tart,
  #[db_rename = "ripe"]
  Ripe,
  #[db_rename = "jammy"]
  Jammy,
  #[db_rename = "baked"]
  Baked,
}