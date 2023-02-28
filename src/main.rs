extern crate redis;
extern crate json;

use json::JsonValue;
use uuid::Uuid;

#[macro_use] extern crate rocket;

#[get("/character")]
fn character() -> &'static str {
    println!("get Character");
    "some characters"
}

#[post("/character")]
fn create_character() -> &'static str {
    "created Character"
}

//1. create new character or load existing character
//2. create new game or load existing game
#[launch]
fn my_server() -> _ {
    println!("Hello, world!");

    let uuid = Uuid::new_v4();

    let data = json::object!{
        id: uuid.to_string(),
        foo: "bar"
    };

    let character: JsonValue = json::object!{
        id: "hossman",
        gameid: uuid.to_string()
    };

    println!("data to store {}", data.dump());
    let mut conn = redis::Client::open("redis://localhost:6379")
    .expect ("invalid connection url")
    .get_connection()
    .expect("failed to connect to Redis");

    //Add/update data to Redis
    let _: () = redis::cmd("SET")
        .arg(uuid.to_string())
        .arg(data.dump())
        .query(&mut conn)
        .expect("failed to execute SET for 'foo'");

    let _: () = redis::cmd("SET")
        .arg("hossman")
        .arg(character.dump())
        .query(&mut conn)
        .expect("failed to execute SET for 'foo'");

    //get data from redis
    let answer: String = redis::cmd("GET")
    .arg(uuid.to_string())
    .query(&mut conn)
    .expect("failed to execute GET for for foo");
    println!("Value for foo {}",answer);


   rocket::build().mount("/", routes![character])
}
