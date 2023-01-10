use crate::schema::users;
use serde::Deserialize;

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub removed: bool,
}
impl<'a> NewUser<'a>{
    pub fn new(
        name: &'a str,
        email: &'a str,
    ) -> NewUser<'a>{
        NewUser {
            name: name,
            email: email,
            removed: false,
        }
    }
}

#[derive(Queryable, Debug, AsChangeset)]
pub struct User{
    pub uuid: i32,
    pub name: String,
    pub email: String,
    pub removed: bool,
}