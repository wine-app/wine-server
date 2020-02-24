use crate::schema::*;

use super::FruitCondition;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "red_analyses"]
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

#[derive(Insertable, AsChangeset)]
#[table_name = "red_analyses"]
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