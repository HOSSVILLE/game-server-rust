extern crate redis;

#[macro_use] extern crate rocket;

//internal mods
mod game_models;
mod character_controller;
mod game_controller;

#[launch]
fn my_server() -> _ {

   rocket::build()
      .mount("/character", routes![character_controller::create_character,character_controller::get_character, character_controller::get_characters])
      .mount("/game", routes![game_controller::create_game, game_controller::get_games])
}
