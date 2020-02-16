#[derive(Queryable, juniper::GraphQLObject)]
#[graphql(description="A blog post")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use super::schema::posts;

#[derive(Insertable, juniper::GraphQLInputObject)]
#[table_name="posts"]
#[graphql(description="A blog post")]
pub struct NewPost {
    pub title: String,
    pub body: String,
}