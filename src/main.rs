extern crate redis;
extern crate rocket_contrib;
extern crate serde_json;
//use json::JsonValue;
use uuid::Uuid;

//use serde::Deseria3ze;

#[macro_use] extern crate rocket;


#[derive(serde::Deserialize, serde::Serialize)]
pub struct User {
  pub name: String,
  pub firstname: String,
  pub surname: String,
  #[serde(skip_deserializing)]
  pub id: String
}

#[post("/character", format = "application/json", data = "<input>")]
fn create_character(input: String) {
    let mut data : User = serde_json::from_str(&input).unwrap();
    
    data.id = Uuid::new_v4().to_string();

    
    let mut conn = redis::Client::open("redis://localhost:6379")
    .expect ("invalid connection url")
    .get_connection()
    .expect("failed to connect to Redis");

    let _: () = redis::cmd("SET")
    .arg(&data.id)
    .arg(serde_json::to_string(&data).unwrap())
    .query(&mut conn)
    .expect("failed to execute SET for 'foo'");

    let answer: String = redis::cmd("GET")
    .arg(data.id)
    .query(&mut conn)
    .expect("failed to execute GET for for foo");
    println!("character saved {}",answer);



}

#[get("/character")]
fn get_characters() {

    let mut conn = redis::Client::open("redis://localhost:6379")
    .expect ("invalid connection url")
    .get_connection()
    .expect("failed to connect to Redis");

    let answer: Vec<String> = redis::cmd("KEYS")
    .arg("*")
    .query(&mut conn)
    .expect("failed to execute GET for for foo");

    println!("character retrieved {:?}",answer);

}

//1. create new character or load existing character
//2. create new game or load existing game
#[launch]
fn my_server() -> _ {

   rocket::build().mount("/", routes![create_character,get_characters])
}
