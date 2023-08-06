use actix_web::web;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

use crate::models::AppState;

use super::BasicOauthToken;

#[derive(Deserialize)]
pub struct NaverUserResult {
    pub resultcode: String,
    pub message: String,
    pub response: NaverUserResponse,
}

#[derive(Deserialize)]
pub struct NaverUserResponse {
    pub id: String,
    pub nickname: String,
    pub email: String,
    pub profile_image: String,
    pub name: Option<String>,
    pub gender: Option<String>,
    pub age: Option<String>,
    pub birthday: Option<String>,
    pub birthyear: Option<String>,
    pub mobile: Option<String>,
}

pub async fn get_naver_oauth_token(
    authorization_code: &str,
    data: &web::Data<AppState>,
) -> Result<BasicOauthToken, Box<dyn Error>> {
    let redirect_url = data.env.naver_oauth_redirect_url.to_owned();
    let client_id = data.env.naver_oauth_client_id.to_owned();
    let client_secret = data.env.naver_oauth_client_secret.to_owned();

    let root_url = "https://nid.naver.com/oauth2.0/token";

    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("redirect_uri", redirect_url.as_str()),
        ("code", authorization_code),
    ];

    let response = client.post(root_url).form(&params).send().await?;

    if response.status().is_success() {
        // let response_value: serde_json::Value = response.json().await?;
        // println!("Response: {:#?}", response_value);
        // let oauth_response = serde_json::from_value::<NaverOauthToken>(response_value)?;
        let oauth_response = response.json::<BasicOauthToken>().await?;
        Ok(oauth_response)
    } else {
        let message = "An error occurred while trying to retrieve the access token.";
        Err(From::from(message))
    }
}

pub async fn get_naver_user(access_token: &str) -> Result<NaverUserResult, Box<dyn Error>> {
    let root_url = "https://openapi.naver.com/v1/nid/me";

    let client = Client::new();

    let response = client
        .get(root_url)
        .header(reqwest::header::USER_AGENT, "blog-rs")
        .bearer_auth(access_token)
        .send()
        .await?;

    if response.status().is_success() {
        // let response_value: serde_json::Value = response.json().await?;
        // println!("Response: {:#?}", response_value);
        // let user_info = serde_json::from_value::<NaverUserResult>(response_value)?;

        let user_info = response.json::<NaverUserResult>().await?;
        Ok(user_info)
    } else {
        // Read the response text to get the error message
        let error_text = response.text().await?;
        println!("Error: {}", error_text);
        let message = "An error occurred while trying to retrieve user information.";
        Err(From::from(message))
    }
}
