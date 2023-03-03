extern crate redis;
extern crate rocket_contrib;
extern crate serde_json;

mod game_models;

use crate::game_models::CreateResponse;
use crate::game_models::GetCharacterRequest;
use crate::game_models::User;

use uuid::Uuid;

#[macro_use] extern crate rocket;

#[post("/character", format = "application/json", data = "<input>")]
fn create_character(input: String) -> String {
    let mut data :  User = serde_json::from_str(&input).unwrap();
    
    let uuid: Uuid = Uuid::new_v4();

    data.id = uuid.to_string();
    
    let mut conn = redis::Client::open("redis://localhost:6379")
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

  return serde_json::to_string(&response).unwrap()
    


}

#[get("/character/<character_id>")]
fn get_character(character_id : String) -> String {

  let data : GetCharacterRequest  = GetCharacterRequest {
    character_id : format!("{}",character_id.as_str())
  };
    
    let mut conn = redis::Client::open("redis://localhost:6379")
    .expect ("invalid connection url")
    .get_connection()
    .expect("failed to connect to Redis");

    let answer: String = redis::cmd("GET")
    .arg(data.character_id)
    .query(&mut conn)
    .expect("failed to execute GET for for foo");

    println!("character retrieved {:?}",answer);

    return answer;

}

#[launch]
fn my_server() -> _ {

   rocket::build().mount("/", routes![create_character,get_character])
}
