use crate::models::User as DbUser;
use crate::models::UserInput as DbUserInput;

#[derive(Debug, juniper::GraphQLObject)]
pub struct User {
  pub id: i32,
  pub facebook_user_id: Option<String>,
  pub google_username: Option<String>,
}

impl From<DbUser> for User {
  fn from(item: DbUser) -> User {
    User {
      id: item.id,
      facebook_user_id: item.facebook_user_id,
      google_username: item.google_username,
    }
  }
}

#[derive(Debug, juniper::GraphQLInputObject)]
pub struct UserInput {
  pub facebook_username: Option<String>,
  pub google_username: Option<String>,
}

impl From<UserInput> for DbUserInput {
  fn from(item: UserInput) -> DbUserInput {
    DbUserInput {
      facebook_user_id: item.facebook_username,
      google_username: item.google_username,
    }
  }
}