use actix_web::web;
use reqwest::{Client, Url};
use serde::Deserialize;
use std::error::Error;

use crate::models::AppState;

use super::GoogleOAuthToken;

#[derive(Deserialize)]
pub struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: Option<String>,
    pub picture: String,
    pub locale: String,
}

pub async fn get_google_oauth_token(
    authorization_code: &str,
    data: &web::Data<AppState>,
) -> Result<GoogleOAuthToken, Box<dyn Error>> {
    let redirect_url = data.env.google_oauth_redirect_url.to_owned();
    let client_secret = data.env.google_oauth_client_secret.to_owned();
    let client_id = data.env.google_oauth_client_id.to_owned();

    let root_url = "https://oauth2.googleapis.com/token";
    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("code", authorization_code),
        ("redirect_uri", redirect_url.as_str()),
    ];
    let response = client.post(root_url).form(&params).send().await?;

    if response.status().is_success() {
        let oauth_response = response.json::<GoogleOAuthToken>().await?;
        Ok(oauth_response)
    } else {
        let message = "An error occurred while trying to retrieve access token.";
        Err(From::from(message))
    }
}

pub async fn get_google_user(
    access_token: &str,
    id_token: &str,
) -> Result<GoogleUserResult, Box<dyn Error>> {
    let client = Client::new();
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
    url.query_pairs_mut().append_pair("alt", "json");
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let response = client.get(url).bearer_auth(id_token).send().await?;

    if response.status().is_success() {
        let user_info = response.json::<GoogleUserResult>().await?;
        Ok(user_info)
    } else {
        let message = "An error occurred while trying to retrieve user information.";
        Err(From::from(message))
    }
}
