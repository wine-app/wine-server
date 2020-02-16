use crate::db::PgPool;
use crate::models::*;
use juniper::FieldResult;

pub struct GraphqlContext {
  // Use your real database pool here.
  pub db_pool: PgPool,
}

impl juniper::Context for GraphqlContext {}

pub struct RootQuery;

#[juniper::object(Context = GraphqlContext)]
impl RootQuery {
  fn apiVersion() -> &str {
    "0.1"
  }

  // Arguments to resolvers can either be simple types or input objects.
  // To gain access to the context, we specify a argument
  // that is a reference to the Context type.
  // Juniper automatically injects the correct context here.
  fn post(context: &GraphqlContext, id: i32) -> FieldResult<Post> {
    use crate::diesel::prelude::*;
    use crate::schema::posts::dsl::*;
    // Get a db connection.
    let connection = context.db_pool.get()?;

    // Execute a db query.

    let result = posts.find(id).get_result::<Post>(&connection)?;

    // // Return the result.
    Ok(result)
  }
}

pub struct RootMutation;

#[juniper::object(Context = GraphqlContext)]
impl RootMutation {
  fn create_post(context: &GraphqlContext, title: String, body: String) -> FieldResult<Post> {
    use crate::create_post;

    // Get a db connection.
    let connection = context.db_pool.get()?;

    Ok(create_post(&connection, title, body))
  }
}
