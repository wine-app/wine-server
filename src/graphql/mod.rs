mod user;
mod wine;
mod composition;
mod red_analysis;
mod white_analysis;
mod wine_color;
mod fruit_condition;
mod root_query;
mod root_mutation;
mod review;

use std::sync::Arc;
use uuid::Uuid;
use juniper::RootNode;

use crate::db::PgPool;
pub use root_query::RootQuery;
pub use root_mutation::RootMutation;

pub struct GraphqlContext {
  pub db_pool: Arc<PgPool>,
  pub user_id: i32,
}

impl juniper::Context for GraphqlContext {}

pub type Schema = RootNode<'static, RootQuery, RootMutation>;