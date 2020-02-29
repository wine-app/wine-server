use actix_web::{get, Error, HttpResponse};
use actix_identity::Identity;
use serde_derive::Serialize;
use std::env;

#[derive(Serialize)]
struct FacebookLogin {
  client_id: String,
  redirect_uri: String,
  state: String,
  response_type: String
}

#[get("/login_page")]
pub async fn login_page(
  id: Identity
) -> Result<HttpResponse, Error> {
  id.forget();
  let my_login = FacebookLogin {
    client_id: env::var("FB_APP_ID").unwrap(),
    redirect_uri: "http://localhost:8080/login".to_string(),
    state: "".to_string(),
    response_type: "code".to_string(),
  };

  let uri = format!("https://www.facebook.com/v6.0/dialog/oauth?{}&scope=email", serde_urlencoded::to_string(&my_login).unwrap());


  Ok(HttpResponse::Ok().content_type("text/html").body(format!("<a href=\"{}\">Facebook Login</a>", uri)))
}