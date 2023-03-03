
#[derive(serde::Deserialize, serde::Serialize)]
pub struct User {
  pub name: String,
  pub firstname: String,
  pub surname: String,
  #[serde(skip_deserializing)]
  pub id: String
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateResponse {
  pub character_id: String
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GetCharacterRequest {
  pub character_id: String
}
