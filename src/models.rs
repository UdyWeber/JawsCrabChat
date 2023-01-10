use crate::schema::users;

#[derive(Insertable)]
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
        is_removed: bool,
    ) -> NewUser<'a>{
        NewUser {
            name: name,
            email: email,
            removed: is_removed,
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