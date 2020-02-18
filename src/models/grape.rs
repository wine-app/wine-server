use crate::schema::*;

#[derive(Identifiable, Insertable, Queryable, Associations, Debug)]
#[table_name = "grapes"]
#[primary_key(name)]
pub struct Grape {
    pub name: String,
}
