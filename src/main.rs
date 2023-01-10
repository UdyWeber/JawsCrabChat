use actix_web::{web, App, HttpServer, get, HttpResponse, HttpRequest};

mod db_config;
mod data_structures;

use db_config::establish_connection;
use data_structures::app_state::{AppState, get_and_update_app_state_data};

async fn greetings(request_data: HttpRequest, data: web::Data<AppState>) -> String {
    let mut threat_message = String::new();

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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_connection = establish_connection();

    let application_data = web::Data::new(AppState::default());

    HttpServer::new(move || {
        let app_data = application_data.clone();
        App::new()
            .app_data(app_data)
            .route("/status", web::get().to(get_and_update_app_state_data))
            .route("/", web::get().to(greetings))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
