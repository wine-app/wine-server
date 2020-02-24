use diesel::prelude::*;
use juniper::FieldResult;

use crate::models::Wine as DbWine;
use crate::models::WineInput as DbWineInput;
use crate::models::Composition as DbComposition;
use crate::PrependError;

use super::GraphqlContext;
use super::wine_color::WineColor;
use super::composition::Composition;

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

impl From<DbWine> for Wine {
  fn from(item: DbWine) -> Wine {
    Wine {
      id: item.id,
      color: item.color.into(),
      name: item.name,
      producer: item.producer,
      vintage: item.vintage,
      region: item.region,
      country: item.country,
      sparkling: item.sparkling,
      alcohol: item.alcohol,
    }
  }
}

#[juniper::object(Context = GraphqlContext)]
impl Wine {
  fn id(&self) -> i32 {
    self.id
  }
  fn color(&self) -> WineColor {
    self.color.clone()
  }
  fn name(&self) -> &str {
    &self.name
  }
  fn producer(&self) -> &str {
    &self.producer
  }
  fn vintage(&self) -> i32 {
    self.vintage
  }
  fn region(&self) -> &str {
    &self.region
  }
  fn country(&self) -> &str {
    &self.country
  }
  fn sparkling(&self) -> bool {
    self.sparkling
  }
  fn alcohol(&self) -> f64 {
    self.alcohol
  }
  fn composition(&self, context: &GraphqlContext) -> FieldResult<Vec<Composition>> {
    use crate::schema::compositions::dsl::*;
    let connection = context.db_pool.get()?;

    let result: Vec<DbComposition> = compositions
      .filter(wine_id.eq(self.id))
      .load(&connection)
      .prepend_err("Error retrieving compositions")?;
    Ok(result.into_iter().map(|x| x.into()).collect())
  }
  // fn analyses(&self, context: &GraphqlContext) -> FieldResult<Vec<Composition>> {
  //   use crate::schema::compositions::dsl::*;
  //   let connection = context.db_pool.get()?;

  //   let result: Vec<DbComposition> = compositions
  //     .filter(wine_id.eq(self.id))
  //     .load(&connection)
  //     .prepend_err("Error retrieving compositions")?;
  //   Ok(result.into_iter().map(|x| x.into()).collect())
  // }
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
pub struct CompositionInput {
  pub grape: String,
  pub percent: i32,
}

#[derive(juniper::GraphQLInputObject, Debug)]
pub struct WineInput {
  pub name: String,
  pub color: WineColor,
  pub producer: String,
  pub vintage: i32,
  pub region: String,
  pub country: String,
  pub sparkling: bool,
  pub alcohol: f64,
  pub composition: Vec<CompositionInput>,
}

impl From<WineInput> for DbWineInput {
  fn from(item: WineInput) -> DbWineInput {
    DbWineInput {
      name: item.name,
      color: item.color.into(),
      producer: item.producer,
      vintage: item.vintage,
      region: item.region,
      country: item.country,
      sparkling: item.sparkling,
      alcohol: item.alcohol,
    }
  }
}
