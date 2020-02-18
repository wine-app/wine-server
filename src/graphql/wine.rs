use juniper::FieldResult;
use diesel::prelude::*;

use crate::graphql::GraphqlContext;
use crate::models::Wine as DbWine;
use crate::models::WineInput as DbWineInput;
use crate::models::WineIntensity as DbWineIntensity;
use crate::models::Composition as DbComposition;

#[derive(juniper::GraphQLEnum, Debug)]
pub enum WineIntensity {
  Delicate,
  Moderate,
  Powerful,
}

impl From<DbWineIntensity> for WineIntensity {
  fn from(item: DbWineIntensity) -> WineIntensity {
    match item {
      DbWineIntensity::Delicate => WineIntensity::Delicate,
      DbWineIntensity::Moderate => WineIntensity::Moderate,
      DbWineIntensity::Powerful => WineIntensity::Powerful,
    }
  }
}

impl From<WineIntensity> for DbWineIntensity {
  fn from(item: WineIntensity) -> DbWineIntensity {
    match item {
      WineIntensity::Delicate => DbWineIntensity::Delicate,
      WineIntensity::Moderate => DbWineIntensity::Moderate,
      WineIntensity::Powerful => DbWineIntensity::Powerful,
    }
  }
}

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

impl From<DbWine> for Wine {
  fn from(item: DbWine) -> Wine {
    Wine {
      id: item.id,
      name: item.name,
      producer: item.producer,
      vintage: item.vintage,
      region: item.region,
      country: item.country,
      sparkling: item.sparkling,
      sweetness: item.sweetness,
      tannin: item.tannin,
      acid: item.acid,
      alcohol: item.alcohol,
      body: item.body,
      intensity: item.intensity.into(),
    }
  }
}

#[derive(Debug, juniper::GraphQLObject)]
pub struct Composition {
  pub grape: String,
  pub percent: i32,
}

impl From<DbComposition> for Composition {
  fn from(item: DbComposition) -> Composition {
    Composition {
      grape: item.grape,
      percent: item.percent,
    }
  }
}

#[juniper::object(Context = GraphqlContext)]
impl Wine {
  fn id(&self) -> i32 {
    self.id
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
  fn sweetness(&self) -> i32 {
    self.sweetness
  }
  fn tannin(&self) -> i32 {
    self.tannin
  }
  fn acid(&self) -> i32 {
    self.acid
  }
  fn alcohol(&self) -> i32 {
    self.alcohol
  }
  fn body(&self) -> i32 {
    self.body
  }
  fn intensity(&self) -> &WineIntensity {
    &self.intensity
  }
  fn composition(&self, context: &GraphqlContext) -> FieldResult<Vec<Composition>> {
    use crate::schema::compositions::dsl::*;
    let connection = context.db_pool.get()?;

    let result: Vec<DbComposition> = compositions.filter(wine_id.eq(self.id)).load(&connection)?;
    Ok(result.into_iter().map(|x| x.into()).collect())
  }
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
pub struct CompositionInput {
  pub grape: String,
  pub percent: i32,
}

#[derive(juniper::GraphQLInputObject, Debug)]
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
  pub composition: Vec<CompositionInput>,
}

impl From<WineInput> for DbWineInput {
  fn from(item: WineInput) -> DbWineInput {
    DbWineInput {
      name: item.name,
      producer: item.producer,
      vintage: item.vintage,
      region: item.region,
      country: item.country,
      sparkling: item.sparkling,
      sweetness: item.sweetness,
      tannin: item.tannin,
      acid: item.acid,
      alcohol: item.alcohol,
      body: item.body,
      intensity: item.intensity.into(),
    }
  }
}

// impl From<Wine> for WineInput {
//   fn from(item: Wine) -> WineInput {
//     WineInput {
      // name: item.name,
      // producer: item.producer,
      // vintage: item.vintage,
      // region: item.region,
      // country: item.country,
      // sparkling: item.sparkling,
      // sweetness: item.sweetness,
      // tannin: item.tannin,
      // acid: item.acid,
      // alcohol: item.alcohol,
      // body: item.body,
      // intensity: item.intensity,
//       composition: item.composition(context: &GraphqlContext)
//     }
//   }
// }
