use serde_derive::Serialize;
use serde_urlencoded;
use reqwest;
use std::env;
use dotenv::dotenv;


#[derive(Serialize)]
struct FacebookLogin {
  client_id: String,
  redirect_uri: String,
  state: String,
  response_type: String
}


#[tokio::main]
async fn main() {
  dotenv().ok();
  let my_login = FacebookLogin {
    client_id: env::var("FB_APP_ID").unwrap(),
    redirect_uri: "http://localhost:8080/login".to_string(),
    state: "".to_string(),
    response_type: "code".to_string(),
  };
  let uri = format!("https://www.facebook.com/v6.0/dialog/oauth?{}&scope=email", serde_urlencoded::to_string(&my_login).unwrap());
  println!("{}", uri);
  let body = reqwest::get(&uri).await.unwrap();
  // println!("{}", body.text().await.unwrap());
}