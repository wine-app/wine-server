mod user;
mod wine;
mod root_query;
mod root_mutation;

use std::sync::Arc;
use uuid::Uuid;
use juniper::RootNode;

use crate::db::PgPool;
pub use root_query::RootQuery;
pub use root_mutation::RootMutation;

pub struct GraphqlContext {
  pub db_pool: Arc<PgPool>,
  pub user_id: Uuid,
}

impl juniper::Context for GraphqlContext {}

pub type Schema = RootNode<'static, RootQuery, RootMutation>;