use diesel::prelude::*;
use juniper::FieldResult;

use crate::models::{WhiteAnalysis as DbWhiteAnalysis, WhiteAnalysisInput as DbWhiteAnalysisInput};
use super::fruit_condition::FruitCondition;

#[derive(juniper::GraphQLObject, Debug)]
pub struct WhiteAnalysis {
  pub id: i32,
  pub wine_id: i32,
  pub apple: i32,
  pub citrus: i32,
  pub stone_fruit: i32,
  pub tropical: i32,
  pub floral: i32,
  pub herbal: i32,
  pub vegetal: i32,
  pub botrytis: i32,
  pub nutty: i32,
  pub lees: i32,
  pub buttery: i32,
  pub organic: i32,
  pub inorganic: i32,
  pub wood: i32,
  pub phenolic: i32,
  pub sweetness: i32,
  pub acid: i32,
  pub alcohol: i32,
  pub fruit_condition: FruitCondition,
}

impl From<DbWhiteAnalysis> for WhiteAnalysis {
  fn from(item: DbWhiteAnalysis) -> WhiteAnalysis {
    WhiteAnalysis {
      id: item.id,
      wine_id: item.wine_id,
      apple: item.apple,
      citrus: item.citrus,
      stone_fruit: item.stone_fruit,
      tropical: item.tropical,
      floral: item.floral,
      herbal: item.herbal,
      vegetal: item.vegetal,
      botrytis: item.botrytis,
      nutty: item.nutty,
      lees: item.lees,
      buttery: item.buttery,
      organic: item.buttery,
      inorganic: item.inorganic,
      wood: item.wood,
      phenolic: item.phenolic,
      sweetness: item.sweetness,
      acid: item.acid,
      alcohol: item.alcohol,
      fruit_condition: item.fruit_condition.into(),
    }
  }
}

#[derive(juniper::GraphQLInputObject, Debug)]
pub struct WhiteAnalysisInput {
  pub wine_id: i32,
  pub apple: i32,
  pub citrus: i32,
  pub stone_fruit: i32,
  pub tropical: i32,
  pub floral: i32,
  pub herbal: i32,
  pub vegetal: i32,
  pub botrytis: i32,
  pub nutty: i32,
  pub lees: i32,
  pub buttery: i32,
  pub organic: i32,
  pub inorganic: i32,
  pub wood: i32,
  pub phenolic: i32,
  pub sweetness: i32,
  pub acid: i32,
  pub alcohol: i32,
  pub fruit_condition: FruitCondition,
}

impl From<WhiteAnalysisInput> for DbWhiteAnalysisInput {
  fn from(item: WhiteAnalysisInput) -> DbWhiteAnalysisInput {
    DbWhiteAnalysisInput {
      wine_id: item.wine_id,
      apple: item.apple,
      citrus: item.citrus,
      stone_fruit: item.stone_fruit,
      tropical: item.tropical,
      floral: item.floral,
      herbal: item.herbal,
      vegetal: item.vegetal,
      botrytis: item.botrytis,
      nutty: item.nutty,
      lees: item.lees,
      buttery: item.buttery,
      organic: item.buttery,
      inorganic: item.inorganic,
      wood: item.wood,
      phenolic: item.phenolic,
      sweetness: item.sweetness,
      acid: item.acid,
      alcohol: item.alcohol,
      fruit_condition: item.fruit_condition.into(),
    }
  }
}
