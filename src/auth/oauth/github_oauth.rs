use actix_web::web;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

use crate::models::AppState;

use super::BasicOauthToken;

#[derive(Deserialize)]
pub struct GitHubUserResult {
    pub login: String,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
}

pub async fn get_github_oauth_token(
    authorization_code: &str,
    data: &web::Data<AppState>,
) -> Result<BasicOauthToken, Box<dyn Error>> {
    let client_id = data.env.github_oauth_client_id.to_owned();
    let client_secret = data.env.github_oauth_client_secret.to_owned();

    let root_url = "https://github.com/login/oauth/access_token";

    let client = Client::new();

    let params = [
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("code", authorization_code),
    ];

    let response = client
        .post(root_url)
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let oauth_response = response.json::<BasicOauthToken>().await?;
        Ok(oauth_response)
    } else {
        let message = "An error occurred while trying to retrieve the access token.";
        Err(From::from(message))
    }
}

pub async fn get_github_user(access_token: &str) -> Result<GitHubUserResult, Box<dyn Error>> {
    let root_url = "https://api.github.com/user";

    let client = Client::new();

    let response = client
        .get(root_url)
        .header(reqwest::header::USER_AGENT, "blog-rs")
        .bearer_auth(access_token)
        .send()
        .await?;

    if response.status().is_success() {
        let user_info = response.json::<GitHubUserResult>().await?;
        Ok(user_info)
    } else {
        // Read the response text to get the error message
        let error_text = response.text().await?;
        println!("Error: {}", error_text);
        let message = "An error occurred while trying to retrieve user information.";
        Err(From::from(message))
    }
}
