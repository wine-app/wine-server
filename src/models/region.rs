use crate::schema::*;

#[derive(Identifiable, Insertable, Queryable, Associations, Debug)]
#[table_name = "regions"]
#[primary_key(name)]
pub struct Region {
    pub name: String,
}
