pub mod github_oauth;
pub mod google_oauth;
pub mod kakao_oauth;
pub mod naver_oauth;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct GoogleOAuthToken {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Deserialize)]
pub struct BasicOauthToken {
    pub access_token: String,
}

pub enum OAuthTokenResponse {
    Google(GoogleOAuthToken),
    GitHub(BasicOauthToken),
    Kakao(BasicOauthToken),
    Naver(BasicOauthToken),
}

pub struct UserInfo {
    pub name: String,
    pub email: String,
    pub photo: Option<String>,
    pub provider: String,
}
