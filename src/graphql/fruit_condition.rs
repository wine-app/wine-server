use crate::models::FruitCondition as DbFruitCondition;

#[derive(juniper::GraphQLEnum, Debug)]
pub enum FruitCondition {
    Tart,
    Ripe,
    Jammy,
    Baked,
}

impl From<FruitCondition> for DbFruitCondition {
  fn from(item: FruitCondition) -> DbFruitCondition {
    match item {
      FruitCondition::Tart => DbFruitCondition::Tart,
      FruitCondition::Ripe => DbFruitCondition::Ripe,
      FruitCondition::Jammy => DbFruitCondition::Jammy,
      FruitCondition::Baked => DbFruitCondition::Baked,
    }
  }
}

impl From<DbFruitCondition> for FruitCondition {
  fn from(item: DbFruitCondition) -> FruitCondition {
    match item {
      DbFruitCondition::Tart => FruitCondition::Tart,
      DbFruitCondition::Ripe => FruitCondition::Ripe,
      DbFruitCondition::Jammy => FruitCondition::Jammy,
      DbFruitCondition::Baked => FruitCondition::Baked,
    }
  }
}
