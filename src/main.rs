extern crate redis;
extern crate rocket_contrib;


#[macro_use] extern crate rocket;
mod game_models;
mod character_controller;

#[launch]
fn my_server() -> _ {

   rocket::build().mount("/", routes![character_controller::create_character,character_controller::get_character])
}
