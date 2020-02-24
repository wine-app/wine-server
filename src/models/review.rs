use crate::schema::*;

use super::User;
use super::Wine;

#[derive(Identifiable, Insertable, AsChangeset, Queryable, Associations, Debug)]
#[belongs_to(User)]
#[belongs_to(Wine)]
#[table_name = "reviews"]
#[primary_key(user_id, wine_id)]
pub struct Review {
  pub user_id: i32,
  pub wine_id: i32,
  pub liked: bool,
}
