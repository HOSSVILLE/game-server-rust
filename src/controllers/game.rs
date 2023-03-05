use crate::models::{GameCreateRequest, Game};

use rocket::http::Status;

extern crate redis;
extern crate serde_json;

use uuid::Uuid;
// All endpoints here will be mounted at base /game

#[post("/", format = "application/json", data = "<data>")]
pub fn create_game(data: String) ->  (Status, String) {
    let req :  GameCreateRequest = serde_json::from_str(&data).unwrap();

    let uuid: Uuid = Uuid::new_v4();

    let game: Game = Game {
        game_id: uuid.to_string(),
        character_id : req.character_id
    };
   
    let mut conn = redis::Client::open("redis://localhost:6379/1")
    .expect ("invalid connection url")
    .get_connection()
    .expect("failed to connect to Redis");


    let _: () = redis::cmd("SET")
    .arg(format!("{}-{}",uuid.to_string(), game.character_id))
    .arg(serde_json::to_string(&game).unwrap())
    .query(&mut conn)
    .expect("failed to execute SET for 'foo'");


    //serde_json::to_string(&game).unwrap()

    return (Status::Created, serde_json::to_string(&game).unwrap());
     
}

#[get("/")]
pub fn get_games() -> (Status, String) {
    
   
    let mut conn = redis::Client::open("redis://localhost:6379/1")
    .expect ("invalid connection url")
    .get_connection()
    .expect("failed to connect to Redis");


    let list : Vec<String>  = redis::cmd("KEYS")
    .arg("*")
    .query(&mut conn)
    .expect("failed to execute SET for 'foo'");

    println!("character saved {:?}",list);

    return (Status::Ok, serde_json::to_string(&list).unwrap())

     
}
