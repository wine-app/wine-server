use crate::schema::*;

#[derive(Identifiable, Insertable, Queryable, Associations, Debug)]
#[table_name = "countries"]
#[primary_key(name)]
pub struct Country {
    pub name: String,
}
