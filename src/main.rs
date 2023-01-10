#[macro_use]
extern crate diesel;
extern crate dotenvy;


mod db_config;

fn main() {
    let connection = db_config::establish_connection();
}
