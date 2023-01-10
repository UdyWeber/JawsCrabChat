use actix_web::{Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInfo {
    name: String,
    email: String,
}

pub async fn add_user_to_app(user_info: web::Json<UserInfo>) -> impl Responder{
    format!(
        "User email: {} and username: {} was added successfully to the app",
        user_info.email,
        user_info.name
    )
}