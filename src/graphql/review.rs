use crate::models::Review as DbReview;

#[derive(Debug, juniper::GraphQLObject)]
pub struct Review {
  pub user_id: i32,
  pub wine_id:i32,
  pub liked: bool,
}

impl From<DbReview> for Review {
  fn from(item: DbReview) -> Review {
    Review {
      user_id: item.user_id,
      wine_id: item.wine_id,
      liked: item.liked,
    }
  }
}
