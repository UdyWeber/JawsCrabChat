use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use crate::crud_operations::user_operations::{add_user_to_database, get_user_by_email};
use crate::db_config::establish_connection;
use crate::models::NewUser;

#[derive(Deserialize)]
pub struct RestUserInfo {
    username: String,
    email: String,
}

pub async fn add_user_to_app(
    rest_user_data: web::Json<RestUserInfo>
) -> impl Responder {
    let connection = &mut establish_connection();

    let new_user = NewUser::new(
        &rest_user_data.username,
        &rest_user_data.email,
    );

    if get_user_by_email(connection, new_user.email).is_some() {
        HttpResponse::BadRequest().body("BAD REQUEST: User Already Exists In Database");
    }

    add_user_to_database(&new_user, connection);

    HttpResponse::Ok().body(format!(
        "User email: {} and username: {} was added successfully to the app",
        new_user.email,
        new_user.name
    ))
}