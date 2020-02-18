use crate::schema::*;

#[derive(Identifiable, Insertable, Queryable, Associations, Debug)]
#[table_name = "producers"]
#[primary_key(name)]
pub struct Producer {
    pub name: String,
}
