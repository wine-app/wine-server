
use crate::schema::*;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub facebook_user_id: Option<String>,
  pub google_username: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct UserInput {
  pub facebook_user_id: Option<String>,
  pub google_username: Option<String>,
}