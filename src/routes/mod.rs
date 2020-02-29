use std::sync::Arc;

mod get_logout;
mod get_login;
mod get_index;
mod post_graphql;
mod get_graphiql;
mod get_login_page;

pub use get_logout::logout;
pub use get_login::login;
pub use get_index::index;
pub use post_graphql::graphql;
pub use get_graphiql::graphiql;
pub use get_login_page::login_page;

use crate::graphql::Schema;
use crate::db::PgPool;

pub struct RequestContext {
  pub schema: Schema,
  pub db_pool: Arc<PgPool>,
}
