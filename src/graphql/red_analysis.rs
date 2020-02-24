use diesel::prelude::*;
use juniper::FieldResult;

use crate::models::{RedAnalysis as DbRedAnalysis, RedAnalysisInput as DbRedAnalysisInput};

use super::fruit_condition::FruitCondition;

#[derive(juniper::GraphQLObject, Debug)]
pub struct RedAnalysis {
  pub id: i32,
  pub wine_id: i32,
  pub red_fruit: i32,
  pub black_fruit: i32,
  pub blue_fruit: i32,
  pub floral: i32,
  pub vegetal: i32,
  pub dried_herbs: i32,
  pub mint: i32,
  pub peppercorn: i32,
  pub mocha: i32,
  pub animalic: i32,
  pub balsamic: i32,
  pub organic: i32,
  pub inorganic: i32,
  pub oak: i32,
  pub tannin: i32,
  pub acid: i32,
  pub alcohol: i32,
  pub fruit_condition: FruitCondition,
}

impl From<DbRedAnalysis> for RedAnalysis {
  fn from(item: DbRedAnalysis) -> RedAnalysis {
    RedAnalysis {
      id: item.id,
      wine_id: item.wine_id,
      red_fruit: item.red_fruit,
      black_fruit: item.black_fruit,
      blue_fruit: item.blue_fruit,
      floral: item.floral,
      vegetal: item.vegetal,
      dried_herbs: item.dried_herbs,
      mint: item.mint,
      peppercorn: item.peppercorn,
      mocha: item.mocha,
      animalic: item.animalic,
      balsamic: item.balsamic,
      organic: item.organic,
      inorganic: item.inorganic,
      oak: item.oak,
      tannin: item.tannin,
      acid: item.acid,
      alcohol: item.alcohol,
      fruit_condition: item.fruit_condition.into(),
    }
  }
}

#[derive(juniper::GraphQLInputObject, Debug)]
pub struct RedAnalysisInput {
  pub wine_id: i32,
  pub red_fruit: i32,
  pub black_fruit: i32,
  pub blue_fruit: i32,
  pub floral: i32,
  pub vegetal: i32,
  pub dried_herbs: i32,
  pub mint: i32,
  pub peppercorn: i32,
  pub mocha: i32,
  pub animalic: i32,
  pub balsamic: i32,
  pub organic: i32,
  pub inorganic: i32,
  pub oak: i32,
  pub tannin: i32,
  pub acid: i32,
  pub alcohol: i32,
  pub fruit_condition: FruitCondition,
}

impl From<RedAnalysisInput> for DbRedAnalysisInput {
  fn from(item: RedAnalysisInput) -> DbRedAnalysisInput {
    DbRedAnalysisInput {
      wine_id: item.wine_id,
      red_fruit: item.red_fruit,
      black_fruit: item.black_fruit,
      blue_fruit: item.blue_fruit,
      floral: item.floral,
      vegetal: item.vegetal,
      dried_herbs: item.dried_herbs,
      mint: item.mint,
      peppercorn: item.peppercorn,
      mocha: item.mocha,
      animalic: item.animalic,
      balsamic: item.balsamic,
      organic: item.organic,
      inorganic: item.inorganic,
      oak: item.oak,
      tannin: item.tannin,
      acid: item.acid,
      alcohol: item.alcohol,
      fruit_condition: item.fruit_condition.into(),
    }
  }
}
