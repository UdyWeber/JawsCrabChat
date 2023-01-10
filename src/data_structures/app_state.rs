use std::sync::Mutex;
use actix_web::web;

pub struct AppState{
    pub app_name: String,
    pub request_counter: Mutex<i32>,
}

impl Default for AppState{
    fn default() -> Self {
        AppState{
            app_name: "JawsCrabChat".into(),
            request_counter: Mutex::new(0)
        }
    }
}

pub async fn get_and_update_app_state_data(app_data: web::Data<AppState>) -> String {
    let mut request_counter = app_data.request_counter.lock().unwrap();
    *request_counter += 1;

    format!(
        "Status: {} is running, and has received {} requests since started running.",
        &app_data.app_name,
        request_counter
    )
}