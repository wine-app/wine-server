use crate::schema::*;

use super::FruitCondition;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "white_analyses"]
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

#[derive(Insertable, AsChangeset)]
#[table_name = "white_analyses"]
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