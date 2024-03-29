use actix_web::{web, App, HttpServer, HttpRequest, guard};
use actix_web::middleware::Logger;

#[macro_use]
extern crate diesel;

mod db_config;
mod data_structures;
mod models;
mod schema;
mod crud_operations;
mod rest_operations;

use data_structures::app_state::{AppState, get_and_update_app_state_data};
use crate::rest_operations::user::add_user_to_app;

async fn greetings(request_data: HttpRequest, data: web::Data<AppState>) -> String {
    let threat_message: String;

    if let Some(real_ip_addrs) = request_data.connection_info().realip_remote_addr(){
        threat_message = format!(
            "Other wise I do have your IP addr: ({:?}) IM GOING AFTER YOU >:(",
            real_ip_addrs
        );
    } else {
        threat_message = "Couldn't find your IP address but IM GOING TO FIND WHO YOU ARE!!!".into()
    }

    get_and_update_app_state_data(data).await;

    format!(
        "Hello fellow User!\n\
        Be Welcome to my first actix web app \n\n\
        It will be as good as my face so dont be judgy I swear I tried my best!! :D\n\
        {}", threat_message
    )
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState::new();
    let application_data = web::Data::new(app_state);

    HttpServer::new(move || {
        let app_data = application_data.clone();
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(app_data)
            .service(
                web::scope("/status")
                    .guard(guard::Header("PermissionTok", "CheckStatus"))
                    .route("", web::get().to(get_and_update_app_state_data))
            )
            .service(
                web::scope("/add_user")
                    .route("", web::post().to(add_user_to_app))
            )
            .route("/", web::to(greetings))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

    // TODO: Create Token System Middleware
    // TODO: Create Alembic Migrations and Schema
    // TODO: Start Chat Connections
    // TODO: Create Message System
    // TODO: Maybe Implement Graphql For Some Interface With Subscription
}
