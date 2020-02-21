use crate::db::PgPool;
use actix_identity::Identity;
use actix_web::{get, web, Error, HttpResponse};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

use super::RequestContext;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum LoginParams {
  Facebook(FacebookAuthLoginInfo),
}

#[derive(Serialize, Deserialize, Debug)]
struct AppClaims {
  user_id: i32
}

#[get("/login")]
pub async fn login(
  context: web::Data<Arc<RequestContext>>,
  id: Identity,
  web::Query(info): web::Query<LoginParams>,
) -> Result<HttpResponse, Error> {
  let user_id = match info {
    LoginParams::Facebook(login_info) => {
      login_facebook(login_info, context.db_pool).await.unwrap()
    },
  };

  if let None = user_id {
    HttpResponse.
  }

  let claims = AppClaims {
    user_id,
  };

  encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;

  Ok(HttpResponse::Ok().content_type("application/json").finish())
}

#[derive(Deserialize, Debug)]
pub struct FacebookAuthLoginInfo {
  code: Option<String>,
  token: Option<String>,
}

async fn login_facebook(
  login_params: FacebookAuthLoginInfo,
  db_pool: Arc<PgPool>
) -> Result<Option<i32>, Box<dyn std::error::Error>> {
  let user_id = if let Some(code) = login_params.code {
    let facebook_access_token = get_facebook_client_access_token(code).await?;
    // println!("Facebook Access Token: {}", facebook_access_token);
    let facebook_user_id = get_facebook_user_id(&facebook_access_token).await?;
    get_user_id_from_facebook_user_id(facebook_user_id, db_pool).await?
  } else {
    return Err("Cannot find facebook code to login".into());
  };
  Ok(user_id)
}

async fn get_user_id_from_facebook_user_id(
  facebook_user_id_input: String,
  db_pool: Arc<PgPool>
) -> Result<Option<i32>, Box<dyn std::error::Error>> {
  use diesel::prelude::*;
  use crate::schema::users::dsl::*;
  use crate::models::User;

  let connection = db_pool.get()?;

  let matching_user = users.filter(facebook_user_id.eq(&facebook_user_id_input)).limit(1).get_result::<User>(&connection)?;
  Ok(Some(matching_user.id))
}

#[derive(Serialize)]
pub struct FacebookAccessTokenRequestParameters {
  client_id: String,
  redirect_uri: String,
  client_secret: String,
  code: String,
}

#[derive(Deserialize, Debug)]
pub struct FacebookAccessTokenResponse {
  access_token: String,
  token_type: String,
  expires_in: i32,
}

async fn get_facebook_client_access_token(code: String) -> Result<String, Box<dyn std::error::Error>> {
  let facebook_access_token_request_params = FacebookAccessTokenRequestParameters {
    client_id: env::var("FB_APP_ID")?,
    redirect_uri: "http://localhost:8080/login".to_string(),
    client_secret: env::var("FB_APP_SECRET")?,
    code: code.to_owned(),
  };
  let uri = format!(
    "https://graph.facebook.com/v6.0/oauth/access_token?{}",
    serde_urlencoded::to_string(&facebook_access_token_request_params)?
  );
  let return_val = reqwest::get(&uri).await?.text().await?;
  let facebook_access_token_response: FacebookAccessTokenResponse =
    serde_json::from_str(&return_val)?;
  Ok(facebook_access_token_response.access_token)
}

#[derive(Serialize)]
struct FacebookTokenInspectRequestParameters {
  input_token: String,
  access_token: String,
}

#[derive(Deserialize, Debug)]
struct FacebookTokenInspectResponse {
  data: FacebookTokenInspectResponseData,
}

#[derive(Deserialize, Debug)]
struct FacebookTokenInspectResponseData {
  app_id: String,
  r#type: String,
  application: String,
  data_access_expires_at: i32,
  expires_at: i32,
  is_valid: bool,
  issued_at: i32,
  scopes: Vec<String>,
  user_id: String,
}

async fn get_facebook_user_id(access_token: &str) -> Result<String, Box<dyn std::error::Error>> {
  let facebook_token_inspect_request_parameters = FacebookTokenInspectRequestParameters {
    input_token: access_token.to_owned(),
    access_token: access_token.to_owned(),
  };
  let uri = format!(
    "https://graph.facebook.com/debug_token?{}",
    serde_urlencoded::to_string(&facebook_token_inspect_request_parameters)?
  );
  let return_val = reqwest::get(&uri).await?.text().await?;
  let facebook_token_inspect_response: FacebookTokenInspectResponse =
    serde_json::from_str(&return_val)?;
  println!("Inspect Response {:?}", facebook_token_inspect_response);
  Ok(facebook_token_inspect_response.data.user_id)
}

#[derive(Deserialize, Debug)]
struct FacebookUserInfo {
  id: String,
  name: String,
  email: String,
}

async fn get_facebook_user_info(access_token: &str, user_id: &str) -> Result<FacebookUserInfo, Box<dyn std::error::Error>> {
  let uri = format!(
    "https://graph.facebook.com/v6.0/{}?fields=email,id,name&access_token={}",
    user_id,
    access_token
  );
  println!("user query {:?}", uri);
  let return_val = reqwest::get(&uri).await?.text().await?;
  let facebook_user_info: FacebookUserInfo =
    serde_json::from_str(&return_val)?;
  Ok(facebook_user_info)
}