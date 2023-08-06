use actix_web::web;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

use crate::models::AppState;

use super::BasicOauthToken;

#[derive(Debug, Deserialize)]
pub struct KakaoUserResult {
    pub id: u64,
    pub has_signed_up: Option<bool>,
    pub connected_at: Option<String>,
    pub synched_at: Option<String>,
    pub properties: Option<HashMap<String, String>>,
    pub kakao_account: Option<KakaoAccount>,
    pub for_partner: Option<Partner>,
}

#[derive(Debug, Deserialize)]
pub struct KakaoAccount {
    pub profile_needs_agreement: Option<bool>,
    pub profile_nickname_needs_agreement: Option<bool>,
    pub profile_image_needs_agreement: Option<bool>,
    pub profile: Option<Profile>,
    pub name_needs_agreement: Option<bool>,
    pub name: Option<String>,
    pub email_needs_agreement: Option<bool>,
    pub is_email_valid: Option<bool>,
    pub is_email_verified: Option<bool>,
    pub email: Option<String>,
    pub age_range_needs_agreement: Option<bool>,
    pub age_range: Option<String>,
    pub birthyear_needs_agreement: Option<bool>,
    pub birthyear: Option<String>,
    pub birthday_needs_agreement: Option<bool>,
    pub birthday: Option<String>,
    pub birthday_type: Option<String>,
    pub gender_needs_agreement: Option<bool>,
    pub gender: Option<String>,
    pub phone_number_needs_agreement: Option<bool>,
    pub phone_number: Option<String>,
    pub ci_needs_agreement: Option<bool>,
    pub ci: Option<String>,
    pub ci_authenticated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub nickname: Option<String>,
    pub thumbnail_image_url: Option<String>,
    pub profile_image_url: Option<String>,
    pub is_default_image: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Partner {
    pub uuid: Option<String>,
}

pub async fn get_kakao_oauth_token(
    authorization_code: &str,
    data: &web::Data<AppState>,
) -> Result<BasicOauthToken, Box<dyn Error>> {
    let redirect_url = data.env.kakao_oauth_redirect_url.to_owned();
    let client_id: String = data.env.kakao_oauth_client_id.to_owned();

    let root_url = "https://kauth.kakao.com/oauth/token";

    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", client_id.as_str()),
        ("redirect_uri", redirect_url.as_str()),
        ("code", authorization_code),
    ];

    let response = client
        .post(root_url)
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        // let response_value: serde_json::Value = response.json().await?;
        // println!("Response: {:#?}", response_value);
        // let oauth_response = serde_json::from_value::<KakaoOauthToken>(response_value)?;
        let oauth_response = response.json::<BasicOauthToken>().await?;
        Ok(oauth_response)
    } else {
        let message = "An error occurred while trying to retrieve the access token.";
        Err(From::from(message))
    }
}

pub async fn get_kakao_user(access_token: &str) -> Result<KakaoUserResult, Box<dyn Error>> {
    let root_url = "https://kapi.kakao.com/v2/user/me";

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
        // let user_info = serde_json::from_value::<KakaoUserResult>(response_value)?;

        let user_info = response.json::<KakaoUserResult>().await?;
        Ok(user_info)
    } else {
        // Read the response text to get the error message
        let error_text = response.text().await?;
        println!("Error: {}", error_text);
        let message = "An error occurred while trying to retrieve user information.";
        Err(From::from(message))
    }
}
