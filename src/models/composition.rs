use crate::schema::*;
use super::Wine;

#[derive(Associations, AsChangeset, Insertable, Queryable, Debug, Clone)]
#[belongs_to(Wine)]
#[table_name = "compositions"]
pub struct Composition {
  pub wine_id: i32,
  pub grape: String,
  pub percent: i32,
}
