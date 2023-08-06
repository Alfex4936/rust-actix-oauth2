use std::error::Error;

use crate::{
    auth::github_oauth::{get_github_oauth_token, get_github_user},
    auth::kakao_oauth::{get_kakao_oauth_token, get_kakao_user},
    auth::naver_oauth::{get_naver_oauth_token, get_naver_user},
    auth::OAuthTokenResponse,
    auth::{
        google_oauth::{get_google_oauth_token, get_google_user},
        UserInfo,
    },
    models::{AppState, QueryCode, TokenClaims, User},
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    error::ErrorBadRequest,
    get, web, HttpResponse, Responder, Result as ActixResult,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::header::LOCATION;
use uuid::Uuid;

// A helper function to simplify the error conversion
fn to_bad_gateway<E: std::fmt::Display>(err: E) -> HttpResponse {
    HttpResponse::BadGateway().json(serde_json::json!({
        "status": "fail",
        "message": err.to_string()
    }))
}

enum OAuthProvider {
    Google,
    GitHub,
    Naver,
    Kakao,
}

fn update_user(user: &mut User, user_info: &UserInfo) {
    user.email = user_info.email.clone();
    user.provider = user_info.provider.clone();
    if let Some(photo) = &user_info.photo {
        user.photo = photo.clone();
    }
    user.updatedAt = Some(Utc::now());
    user.provider = user_info.provider.clone();
    user.name = user_info.name.clone();
}

async fn find_or_create_user(user_info: UserInfo, vec: &mut Vec<User>) -> String {
    let email = user_info.email.to_lowercase();
    match vec.iter_mut().find(|user| user.email == email) {
        Some(user) => {
            update_user(user, &user_info);
            user.id.clone().unwrap()
        }
        None => {
            let datetime = Utc::now();
            let id = Uuid::new_v4().to_string();
            vec.push(User {
                id: Some(id.clone()),
                name: user_info.name,
                verified: true,
                email,
                provider: user_info.provider,
                role: "user".to_string(),
                password: "".to_string(),
                photo: user_info.photo.unwrap_or("default.png".to_string()),
                createdAt: Some(datetime),
                updatedAt: Some(datetime),
            });
            id
        }
    }
}

async fn fetch_user_info(
    provider: &OAuthProvider,
    token: &str,
    id_token: Option<&str>,
) -> Result<UserInfo, Box<dyn Error>> {
    match provider {
        OAuthProvider::Google => {
            let google_user = get_google_user(token, id_token.unwrap_or_default()).await?;
            Ok(UserInfo {
                name: google_user.name,
                email: google_user.email,
                photo: Some(google_user.picture),
                provider: "Google".to_string(),
            })
        }
        OAuthProvider::GitHub => {
            let github_user = get_github_user(token).await?;
            Ok(UserInfo {
                name: github_user.login,
                email: github_user.email,
                photo: Some(github_user.avatar_url),
                provider: "GitHub".to_string(),
            })
        }
        OAuthProvider::Kakao => {
            let kakao_user = get_kakao_user(token).await?;
            let kakao_account_ref = kakao_user.kakao_account.as_ref();
            let photo = kakao_account_ref
                .and_then(|account| account.profile.as_ref())
                .and_then(|profile| profile.thumbnail_image_url.as_deref())
                .unwrap_or("default.png");

            let name = kakao_account_ref
                .and_then(|account| account.profile.as_ref())
                .and_then(|profile| profile.nickname.as_deref())
                .unwrap_or("Unknown");

            Ok(UserInfo {
                name: name.to_string(),
                email: kakao_account_ref
                    .and_then(|account| account.email.as_deref())
                    .unwrap_or_default()
                    .to_string(),
                photo: Some(photo.to_string()),
                provider: "Kakao".to_string(),
            })
        }
        OAuthProvider::Naver => {
            let naver_user = get_naver_user(token).await?;
            Ok(UserInfo {
                name: naver_user.response.nickname,
                email: naver_user.response.email,
                photo: Some(naver_user.response.profile_image),
                provider: "Naver".to_string(),
            })
        }
    }
}

#[get("/sessions/oauth/{provider}")]
async fn oauth_handler(
    path: web::Path<(String,)>,
    query: web::Query<QueryCode>,
    data: web::Data<AppState>,
) -> ActixResult<impl Responder> {
    let provider = match path.into_inner().0.as_str() {
        "google" => OAuthProvider::Google,
        "github" => OAuthProvider::GitHub,
        "kakao" => OAuthProvider::Kakao,
        "naver" => OAuthProvider::Naver,
        _ => return Ok(HttpResponse::BadRequest().finish()),
    };

    let code = &query.code;
    let state = &query.state;

    if code.is_empty() {
        return Err(ErrorBadRequest("Authorization code not provided!"));
    }

    let token_response = match provider {
        OAuthProvider::Google => get_google_oauth_token(code.as_str(), &data)
            .await
            .map(OAuthTokenResponse::Google),
        OAuthProvider::GitHub => get_github_oauth_token(code.as_str(), &data)
            .await
            .map(OAuthTokenResponse::GitHub),
        OAuthProvider::Kakao => get_kakao_oauth_token(code.as_str(), &data)
            .await
            .map(OAuthTokenResponse::Kakao),
        OAuthProvider::Naver => get_naver_oauth_token(code.as_str(), &data)
            .await
            .map(OAuthTokenResponse::Naver),
    }
    .map_err(to_bad_gateway)
    .unwrap();

    // Deal OAuth2 providers
    let user_info = match token_response {
        OAuthTokenResponse::Google(token_response) => {
            fetch_user_info(
                &provider,
                &token_response.access_token,
                Some(&token_response.id_token),
            )
            .await?
        }
        OAuthTokenResponse::GitHub(token_response) => {
            fetch_user_info(&provider, &token_response.access_token, None).await?
        }
        OAuthTokenResponse::Naver(token_response) => {
            fetch_user_info(&provider, &token_response.access_token, None).await?
        }
        OAuthTokenResponse::Kakao(token_response) => {
            fetch_user_info(&provider, &token_response.access_token, None).await?
        }
    };

    let mut vec = match data.db.lock() {
        Ok(v) => v,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": "Database error"})))
        }
    };

    let user_id = find_or_create_user(user_info, &mut vec).await;

    let now = Utc::now();
    let token = match encode(
        &Header::default(),
        &TokenClaims {
            sub: user_id,
            exp: (now + Duration::minutes(data.env.jwt_max_age)).timestamp() as usize,
            iat: now.timestamp() as usize,
        },
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    ) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(ActixWebDuration::new(60 * data.env.jwt_max_age, 0))
        .http_only(true)
        .finish();

    Ok(HttpResponse::Found()
        .append_header((LOCATION, format!("{}{}", data.env.client_origin, state)))
        .cookie(cookie)
        .finish())
}
