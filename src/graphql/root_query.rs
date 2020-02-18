use juniper::FieldResult;
use diesel::prelude::*;

use crate::models::*;
use crate::graphql::GraphqlContext;
use super::wine::Wine as GraphQLWine;

pub struct RootQuery;

#[juniper::object(Context = GraphqlContext)]
impl RootQuery {
  fn apiVersion() -> &str {
    env!("CARGO_PKG_VERSION")
  }

  fn grape(context: &GraphqlContext, name: String) -> FieldResult<String> {
    use crate::schema::grapes::dsl::*;
    let connection = context.db_pool.get()?;

    let result = grapes.find(name).get_result::<Grape>(&connection)?.name;
    Ok(result)
  }

  fn grapes(context: &GraphqlContext) -> FieldResult<Vec<String>> {
    use crate::schema::grapes::dsl::*;
    let connection = context.db_pool.get()?;

    let items: Vec<Grape> = grapes.load(&connection)?;

    let result = items.into_iter().map(|x| x.name).collect();
    Ok(result)
  }

  fn country(context: &GraphqlContext, name: String) -> FieldResult<String> {
    use crate::schema::countries::dsl::*;
    let connection = context.db_pool.get()?;

    let result = countries.find(name).get_result::<Country>(&connection)?.name;
    Ok(result)
  }

  fn countries(context: &GraphqlContext) -> FieldResult<Vec<String>> {
    use crate::schema::countries::dsl::*;
    let connection = context.db_pool.get()?;

    let items: Vec<Country> = countries.load(&connection)?;

    let result = items.into_iter().map(|x| x.name).collect();
    Ok(result)
  }

  fn region(context: &GraphqlContext, name: String) -> FieldResult<String> {
    use crate::schema::regions::dsl::*;
    let connection = context.db_pool.get()?;

    let result = regions.find(name).get_result::<Region>(&connection)?.name;
    Ok(result)
  }

  fn regions(context: &GraphqlContext) -> FieldResult<Vec<String>> {
    use crate::schema::regions::dsl::*;
    let connection = context.db_pool.get()?;

    let items: Vec<Region> = regions.load(&connection)?;

    let result = items.into_iter().map(|x| x.name).collect();
    Ok(result)
  }

  fn producer(context: &GraphqlContext, name: String) -> FieldResult<String> {
    use crate::schema::producers::dsl::*;
    let connection = context.db_pool.get()?;

    let result = producers.find(name).get_result::<Producer>(&connection)?.name;
    Ok(result)
  }

  fn producers(context: &GraphqlContext) -> FieldResult<Vec<String>> {
    use crate::schema::producers::dsl::*;
    let connection = context.db_pool.get()?;

    let items: Vec<Producer> = producers.load(&connection)?;

    let result = items.into_iter().map(|x| x.name).collect();
    Ok(result)
  }

  fn wine(context: &GraphqlContext, id: i32) -> FieldResult<GraphQLWine> {
    use crate::schema::wines::dsl::*;
    let connection = context.db_pool.get()?;

    let result = wines.find(id).get_result::<Wine>(&connection)?.into();
    Ok(result)
  }

  fn wines(context: &GraphqlContext) -> FieldResult<Vec<GraphQLWine>> {
    use crate::schema::wines::dsl::*;
    let connection = context.db_pool.get()?;

    let result: Vec<Wine> = wines.load(&connection)?;
    Ok(result.into_iter().map(|x| x.into()).collect())
  }
}