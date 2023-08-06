use crate::{
    auth::token_guard::AuthenticationGuard,
    models::{AppState, LoginUserSchema, RegisterUserSchema, TokenClaims, User},
    responses::{FilteredUser, UserData, UserResponse},
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpResponse, Responder,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

use crate::handlers::oauth_handler::oauth_handler;

const MESSAGE: &str = "OK";

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}
#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    match data.db.lock() {
        Err(_) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error", "message": "Database error"})),
        Ok(mut vec) => {
            if vec.iter().any(|user| user.email == body.email) {
                return HttpResponse::Conflict()
                    .json(serde_json::json!({"status": "fail", "message": "Email already exist"}));
            }

            let uuid_id = Uuid::new_v4();
            let datetime = Utc::now();

            let user = User {
                id: Some(uuid_id.to_string()),
                name: body.name.to_owned(),
                verified: false,
                email: body.email.to_owned().to_lowercase(),
                provider: "local".to_string(),
                role: "user".to_string(),
                password: body.password.to_string(),
                photo: "default.png".to_string(),
                createdAt: Some(datetime),
                updatedAt: Some(datetime),
            };

            vec.push(user.clone());

            let json_response = UserResponse {
                status: "success".to_string(),
                data: UserData {
                    user: user_to_response(&user),
                },
            };

            HttpResponse::Ok().json(json_response)
        }
    }
}

#[post("/auth/login")]
async fn login_user_handler(
    body: web::Json<LoginUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let vec = data.db.lock().unwrap();
    let user_opt = vec
        .iter()
        .find(|user| user.email == body.email.to_lowercase());

    match user_opt {
        None => HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "fail", "message": "Invalid email or password"})),
        Some(user) => match user.provider.as_str() {
            "Google" => HttpResponse::Unauthorized().json(
                serde_json::json!({"status": "fail", "message": "Use Google OAuth2 instead"}),
            ),
            "GitHub" => HttpResponse::Unauthorized()
                .json(serde_json::json!({"status": "fail", "message": "Use GitHub OAuth instead"})),
            "Kakao" => HttpResponse::Unauthorized()
                .json(serde_json::json!({"status": "fail", "message": "Use Kakao OAuth instead"})),
            "Naver" => HttpResponse::Unauthorized()
                .json(serde_json::json!({"status": "fail", "message": "Use Naver OAuth instead"})),
            _ => {
                let jwt_secret = data.env.jwt_secret.to_owned();
                let now = Utc::now();
                let iat = now.timestamp() as usize;
                let exp = (now + Duration::minutes(data.env.jwt_max_age)).timestamp() as usize;
                let claims: TokenClaims = TokenClaims {
                    sub: user.id.as_ref().unwrap().to_string(),
                    exp,
                    iat,
                };

                match encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(jwt_secret.as_ref()),
                ) {
                    Ok(token) => {
                        let cookie = Cookie::build("token", token.clone())
                            .path("/")
                            .max_age(ActixWebDuration::new(60 * data.env.jwt_max_age, 0))
                            .http_only(true)
                            .finish();

                        HttpResponse::Ok()
                            .cookie(cookie)
                            .json(serde_json::json!({"status": "success", "token": token}))
                    }
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
        },
    }
}

#[get("/auth/logout")]
async fn logout_handler(_: AuthenticationGuard) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({"status": "success"}))
}

#[get("/users/me")]
async fn get_me_handler(
    auth_guard: AuthenticationGuard,
    data: web::Data<AppState>,
) -> impl Responder {
    let vec = data.db.lock().unwrap();

    let user = vec
        .iter()
        .find(|user| user.id == Some(auth_guard.user_id.to_owned()));

    let json_response = UserResponse {
        status: "success".to_string(),
        data: UserData {
            user: user_to_response(user.unwrap()),
        },
    };

    HttpResponse::Ok().json(json_response)
}

pub fn user_to_response(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.to_owned().unwrap(),
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        verified: user.verified.to_owned(),
        photo: user.photo.to_owned(),
        provider: user.provider.to_owned(),
        role: user.role.to_owned(),
        createdAt: user.createdAt.unwrap(),
        updatedAt: user.updatedAt.unwrap(),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(register_user_handler)
        .service(login_user_handler)
        .service(logout_handler)
        .service(get_me_handler)
        .service(oauth_handler);

    conf.service(scope);
}
