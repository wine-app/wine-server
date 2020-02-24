use juniper::FieldResult;
use diesel::prelude::*;

use crate::models::*;
use crate::graphql::GraphqlContext;
use super::wine::Wine as GraphQLWine;
use super::user::User as GraphQLUser;
use super::red_analysis::RedAnalysis as GraphQLRedAnalysis;
use super::white_analysis::WhiteAnalysis as GraphQLWhiteAnalysis;
use super::review::Review as GraphQLReview;


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

  fn user(context: &GraphqlContext, id: i32) -> FieldResult<GraphQLUser> {
    use crate::schema::users::dsl::*;
    let connection = context.db_pool.get()?;

    let result = users.find(id).get_result::<User>(&connection)?.into();
    Ok(result)
  }

  fn users(context: &GraphqlContext) -> FieldResult<Vec<GraphQLUser>> {
    use crate::schema::users::dsl::*;
    let connection = context.db_pool.get()?;

    let result: Vec<User> = users.load(&connection)?;
    Ok(result.into_iter().map(|x| x.into()).collect())
  }

  fn red_analyses(context: &GraphqlContext) -> FieldResult<Vec<GraphQLRedAnalysis>> {
    use crate::schema::red_analyses::dsl::*;
    let connection = context.db_pool.get()?;

    let result: Vec<RedAnalysis> = red_analyses.load(&connection)?;
    Ok(result.into_iter().map(|x| x.into()).collect())
  }

  fn white_analyses(context: &GraphqlContext) -> FieldResult<Vec<GraphQLWhiteAnalysis>> {
    use crate::schema::white_analyses::dsl::*;
    let connection = context.db_pool.get()?;

    let result: Vec<WhiteAnalysis> = white_analyses.load(&connection)?;
    Ok(result.into_iter().map(|x| x.into()).collect())
  }

  fn reviews_for_user(context: &GraphqlContext) -> FieldResult<Vec<GraphQLReview>> {
    use crate::schema::reviews::dsl::*;
    let connection = context.db_pool.get()?;

    let result: Vec<Review> = reviews.filter(user_id.eq(&context.user_id)).load(&connection)?;
    Ok(result.into_iter().map(|x| x.into()).collect())
  }

  fn reviews_for_wine(context: &GraphqlContext, wine_id: i32) -> FieldResult<Vec<GraphQLReview>> {
    use crate::schema::reviews::dsl::*;
    let connection = context.db_pool.get()?;

    let result: Vec<Review> = reviews.filter(wine_id.eq(&wine_id)).load(&connection)?;
    Ok(result.into_iter().map(|x| x.into()).collect())
  }
}