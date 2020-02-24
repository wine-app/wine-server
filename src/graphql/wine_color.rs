use crate::models::WineColor as DbWineColor;

#[derive(juniper::GraphQLEnum, Debug, Clone)]
pub enum WineColor {
    Red,
    White,
}

impl From<WineColor> for DbWineColor {
  fn from(item: WineColor) -> DbWineColor {
    match item {
      WineColor::Red => DbWineColor::Red,
      WineColor::White => DbWineColor::White,
    }
  }
}

impl From<DbWineColor> for WineColor {
  fn from(item: DbWineColor) -> WineColor {
    match item {
      DbWineColor::Red => WineColor::Red,
      DbWineColor::White => WineColor::White,
    }
  }
}
