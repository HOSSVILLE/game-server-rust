extern crate redis;

#[macro_use] extern crate rocket;

//internal mods
mod game_models;
mod controllers;

#[launch]
fn my_server() -> _ {

   rocket::build()
      .mount("/character", routes![controllers::character::create_character,controllers::character::get_character, controllers::character::get_characters])
      .mount("/game", routes![controllers::game::create_game, controllers::game::get_games])
}
