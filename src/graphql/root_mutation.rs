use diesel::pg::upsert::excluded;
use diesel::prelude::*;
use juniper::FieldResult;

use super::*;
use crate::graphql::wine::*;
use crate::graphql::user::*;
use crate::PrependError;

pub struct RootMutation;

#[juniper::object(Context = GraphqlContext)]
impl RootMutation {
  fn create_user(context: &GraphqlContext, user: UserInput) -> FieldResult<User> {
    use crate::models::{User as DbUser, UserInput as DbUserInput};
    use crate::schema::users;
    let connection = context.db_pool.get()?;

    let new_item: DbUserInput = user.into();

    let inserted_item = diesel::insert_into(users::table)
      .values(new_item)
      .get_result::<DbUser>(&connection)
      .prepend_err("Error creating new user")?;

    Ok(inserted_item.into())
  }

  fn delete_user(context: &GraphqlContext, id: i32) -> FieldResult<User> {
    use crate::models::User as DbUser;
    use crate::schema::users::dsl::users;
    let connection = context.db_pool.get()?;

    let result: DbUser = diesel::delete(users.find(id))
      .get_result(&connection)
      .prepend_err("Error deleting a user")?;

    Ok(result.into())
  }

  fn update_user(
    context: &GraphqlContext,
    id: i32,
    update: UserInput,
  ) -> FieldResult<User> {
    use crate::models::{User as DbUser, UserInput as DbUserInput};
    use crate::schema::users::dsl::users;
    let connection = context.db_pool.get()?;

    let update_item: DbUserInput = update.into();
    let result: DbUser = diesel::update(users.find(id))
      .set(update_item)
      .get_result(&connection)
      .prepend_err("Error updating a user")?;

    Ok(result.into())
  }

  fn create_grape(context: &GraphqlContext, name: String) -> FieldResult<String> {
    use crate::models::Grape as DbGrape;
    use crate::schema::grapes;
    let connection = context.db_pool.get()?;

    let new_item = DbGrape { name };

    let inserted_item = diesel::insert_into(grapes::table)
      .values(new_item)
      .get_result::<DbGrape>(&connection)
      .prepend_err("Error creating new grape")?;

    Ok(inserted_item.name)
  }

  fn delete_grape(context: &GraphqlContext, name: String) -> FieldResult<String> {
    use crate::models::Grape as DbGrape;
    use crate::schema::grapes::dsl::grapes;
    let connection = context.db_pool.get()?;

    let result: DbGrape = diesel::delete(grapes.find(name))
      .get_result(&connection)
      .prepend_err("Error deleting a grape")?;

    Ok(result.name)
  }

  fn update_grape(
    context: &GraphqlContext,
    old_name: String,
    new_name: String,
  ) -> FieldResult<String> {
    use crate::models::Grape as DbGrape;
    use crate::schema::grapes::dsl::{grapes, name};
    let connection = context.db_pool.get()?;

    let result: DbGrape = diesel::update(grapes.find(old_name))
      .set(name.eq(new_name))
      .get_result(&connection)
      .prepend_err("Error updating a grape")?;

    Ok(result.name)
  }

  fn create_country(context: &GraphqlContext, name: String) -> FieldResult<String> {
    use crate::models::Country as DbCountry;
    use crate::schema::countries;
    let connection = context.db_pool.get()?;

    let inserted_item = diesel::insert_into(countries::table)
      .values(DbCountry { name })
      .get_result::<DbCountry>(&connection)
      .prepend_err("Error creating new country")?;

    Ok(inserted_item.name)
  }

  fn delete_country(context: &GraphqlContext, name: String) -> FieldResult<String> {
    use crate::models::Country as DbCountry;
    use crate::schema::countries::dsl::countries;
    let connection = context.db_pool.get()?;

    let result: DbCountry = diesel::delete(countries.find(name))
      .get_result(&connection)
      .prepend_err("Error deleteing a country")?;

    Ok(result.name)
  }

  fn update_country(
    context: &GraphqlContext,
    old_name: String,
    new_name: String,
  ) -> FieldResult<String> {
    use crate::models::Country as DbCountry;
    use crate::schema::countries::dsl::{countries, name};
    let connection = context.db_pool.get()?;

    let result: DbCountry = diesel::update(countries.find(old_name))
      .set(name.eq(new_name))
      .get_result(&connection)
      .prepend_err("Error updating a country")?;

    Ok(result.name)
  }

  fn create_region(context: &GraphqlContext, name: String) -> FieldResult<String> {
    use crate::models::Region as DbRegion;
    use crate::schema::regions;
    let connection = context.db_pool.get()?;

    let inserted_item = diesel::insert_into(regions::table)
      .values(DbRegion { name })
      .get_result::<DbRegion>(&connection)
      .prepend_err("Error creating new region")?;

    Ok(inserted_item.name)
  }

  fn delete_region(context: &GraphqlContext, name: String) -> FieldResult<String> {
    use crate::models::Region as DbRegion;
    use crate::schema::regions::dsl::regions;
    let connection = context.db_pool.get()?;

    let result: DbRegion = diesel::delete(regions.find(name))
      .get_result(&connection)
      .prepend_err("Error deleting a region")?;

    Ok(result.name)
  }

  fn update_region(
    context: &GraphqlContext,
    old_name: String,
    new_name: String,
  ) -> FieldResult<String> {
    use crate::models::Region as DbRegion;
    use crate::schema::regions::dsl::{name, regions};
    let connection = context.db_pool.get()?;

    let result: DbRegion = diesel::update(regions.find(old_name))
      .set(name.eq(new_name))
      .get_result(&connection)
      .prepend_err("Error updating a region")?;

    Ok(result.name)
  }

  fn create_producer(context: &GraphqlContext, name: String) -> FieldResult<String> {
    use crate::models::Producer as DbProducer;
    use crate::schema::producers;
    let connection = context.db_pool.get()?;

    let inserted_item = diesel::insert_into(producers::table)
      .values(DbProducer { name })
      .get_result::<DbProducer>(&connection)
      .prepend_err("Error creating new producer")?;

    Ok(inserted_item.name)
  }

  fn delete_producer(context: &GraphqlContext, name: String) -> FieldResult<String> {
    use crate::models::Producer as DbProducer;
    use crate::schema::producers::dsl::producers;
    let connection = context.db_pool.get()?;

    let result: DbProducer = diesel::delete(producers.find(name))
      .get_result(&connection)
      .prepend_err("Error deleting a producer")?;

    Ok(result.name)
  }

  fn update_producer(
    context: &GraphqlContext,
    old_name: String,
    new_name: String,
  ) -> FieldResult<String> {
    use crate::models::Producer as DbProducer;
    use crate::schema::producers::dsl::{name, producers};
    let connection = context.db_pool.get()?;

    let result: DbProducer = diesel::update(producers.find(old_name))
      .set(name.eq(new_name))
      .get_result(&connection)
      .prepend_err("Error updating a producer")?;

    Ok(result.name)
  }

  fn create_wine(context: &GraphqlContext, input: WineInput) -> FieldResult<Wine> {
    use crate::models::Composition as DbComposition;
    use crate::models::Wine as DbWine;
    use crate::models::WineInput as DbWineInput;
    use crate::schema::compositions;
    use crate::schema::wines;
    // use ndarray;

    let connection = context.db_pool.get()?;

    let composition = input.composition.clone();
    let new_wine: DbWineInput = input.into();

    // let all_wines: Vec<DbWine> = wines::dsl::wines.load(&connection)?;
    // let num_wines = all_wines.len();

    // let A = {
    //   let row_magnitude: f64 = [
    //     new_wine.sweetness as f64,
    //     new_wine.tannin as f64,
    //     new_wine.acid as f64,
    //     new_wine.alcohol as f64,
    //     new_wine.body as f64,
    //   ]
    //   .iter()
    //   .sum();

    //   ndarray::Array1::<f64>::from(vec![
    //     new_wine.sweetness as f64,
    //     new_wine.tannin as f64,
    //     new_wine.acid as f64,
    //     new_wine.alcohol as f64,
    //     new_wine.body as f64,
    //   ]) / row_magnitude
    // };

    // let M = unsafe {
    //   let mut M = ndarray::Array2::<f64>::uninitialized((num_wines, 5));
    //   all_wines
    //     .into_iter()
    //     .enumerate()
    //     .for_each(|(col_ind, wine)| {
    //       let row_magnitude: f64 = [
    //         wine.sweetness as f64,
    //         wine.tannin as f64,
    //         wine.acid as f64,
    //         wine.alcohol as f64,
    //         wine.body as f64,
    //       ]
    //       .iter()
    //       .sum();

    //       M[(0, col_ind)] = wine.sweetness as f64 / row_magnitude;
    //       M[(1, col_ind)] = wine.tannin as f64 / row_magnitude;
    //       M[(2, col_ind)] = wine.acid as f64 / row_magnitude;
    //       M[(3, col_ind)] = wine.alcohol as f64 / row_magnitude;
    //       M[(4, col_ind)] = wine.body as f64 / row_magnitude;
    //     });
    //   M
    // };
    // println!("{}", A.dot(&M.t()));

    connection.transaction(|| {
      let wine_insert_result = diesel::insert_into(wines::table)
        .values(new_wine)
        .get_result::<DbWine>(&connection)
        .prepend_err("Error creating new wine")?;

      let composition_insert_result = diesel::insert_into(compositions::table)
        .values(
          composition
            .into_iter()
            .map(|x| DbComposition {
              wine_id: wine_insert_result.id,
              grape: x.grape,
              percent: x.percent,
            })
            .collect::<Vec<DbComposition>>(),
        )
        .get_result::<DbComposition>(&connection)
        .prepend_err("Error inserting compositions")?;

      Ok(wine_insert_result.into())
    })
  }

  fn delete_wine(context: &GraphqlContext, id: i32) -> FieldResult<i32> {
    use crate::models::Wine as DbWine;
    use crate::schema::wines::dsl::wines;
    let connection = context.db_pool.get()?;

    let result: DbWine = diesel::delete(wines.find(id))
      .get_result(&connection)
      .prepend_err("Error deleting a wine")?;

    Ok(result.id)
  }

  fn update_wine(context: &GraphqlContext, id: i32, update: WineInput) -> FieldResult<Wine> {
    use crate::models::Composition as DbComposition;
    use crate::models::Wine as DbWine;
    use crate::models::WineInput as DbWineInput;
    use crate::schema::compositions;
    use crate::schema::compositions::dsl::{grape, percent, wine_id};
    use crate::schema::wines::dsl::wines;
    let connection = context.db_pool.get()?;

    let input_compositions = update.composition.clone();
    let update_item: DbWineInput = update.into();

    connection.transaction(|| {
      let inserted_wine = diesel::update(wines.find(id))
        .set(&update_item)
        .get_result::<DbWine>(&connection)
        .prepend_err("Error updating a wine")?;

      let db_input_compositions = input_compositions
        .into_iter()
        .map(|x| DbComposition {
          wine_id: id,
          grape: x.grape,
          percent: x.percent,
        })
        .collect::<Vec<DbComposition>>();

      let updated_compositions: Vec<DbComposition> = diesel::insert_into(compositions::table)
        .values(db_input_compositions)
        .on_conflict((wine_id, grape))
        .do_update()
        .set(percent.eq(excluded(percent)))
        .get_results(&connection)
        .prepend_err("Error updating a wine")?;

      Ok(inserted_wine.into())
    })
  }
}
