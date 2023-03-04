
use crate::game_models::{GetCharacterRequest, User, CreateResponse};

extern crate redis;
extern crate serde_json;

use rocket::http::Status;
use uuid::Uuid;
// all endpoints mounted at '/character'

#[get("/<character_id>")]
pub fn get_character(character_id : String) -> (Status, String) {

  let data : GetCharacterRequest  = GetCharacterRequest {
    character_id : format!("{}",character_id.as_str())
  };
    
    let mut conn = redis::Client::open("redis://localhost:6379/0")
    .expect ("invalid connection url")
    .get_connection()
    .unwrap();

    //let answer: String = conn.json_get(data.character_id,".");
    let answer: String = redis::cmd("GET")
    .arg(data.character_id)
    .query(&mut conn)
    .expect("failed to execute GET for for foo");

    println!("character retrieved {:?}",answer);

    return (Status::Ok, serde_json::to_string(&answer).unwrap())

}

#[get("/")]
pub fn get_characters() -> (Status, String) {

    let mut conn = redis::Client::open("redis://localhost:6379/0")
    .expect ("invalid connection url")
    .get_connection()
    .unwrap();

    //let answer: String = conn.json_get(data.character_id,".");
    let answer: Vec<String> = redis::cmd("keys")
    .arg("*")
    .query(&mut conn)
    .expect("failed to execute GET for for foo");

    println!("character retrieved {:?}",answer);

    return (Status::Ok, serde_json::to_string(&answer).unwrap())

}

#[post("/", format = "application/json", data = "<input>")]
pub fn create_character(input: String) -> (Status, String) {
    let mut data :  User = serde_json::from_str(&input).unwrap();
    
    let uuid: Uuid = Uuid::new_v4();

    data.id = uuid.to_string();
    
    let mut conn = redis::Client::open("redis://localhost:6379/0")
    .expect ("invalid connection url")
    .get_connection()
    .expect("failed to connect to Redis");

    let _: () = redis::cmd("SET")
    .arg(uuid.to_string())
    .arg(serde_json::to_string(&data).unwrap())
    .query(&mut conn)
    .expect("failed to execute SET for 'foo'");

    let answer: String = redis::cmd("GET")
    .arg(uuid.to_string())
    .query(&mut conn)
    .expect("failed to execute GET for for foo");
    println!("character saved {}",answer);

    let response: CreateResponse = CreateResponse {
      character_id: uuid.to_string()
    };

  return (Status::Created, serde_json::to_string(&response).unwrap())
    


}
