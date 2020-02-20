mod wine;
mod root_query;
mod root_mutation;

use crate::db::PgPool;
pub use root_query::RootQuery;
pub use root_mutation::RootMutation;


pub struct GraphqlContext {
  pub db_pool: PgPool,
}

impl juniper::Context for GraphqlContext {}
