# 🚀 OAuth2 Login Demo

<img src="https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbo7ZvX%2Fbtsp64YhCDP%2Fndp7pA0pKA2WkiNpH9yAL0%2Fimg.png" width="300" height="340">


This project serves as a demo for implementing OAuth2 login functionality using multiple providers like Kakao, Naver, Google, and GitHub. Along with OAuth authentication, it also demonstrates how to create users in the database after successful authentication.

이 프로젝트는 카카오, 네이버, 구글, 깃허브와 같은 여러 제공업체를 사용하여 OAuth2 로그인 기능을 구현하기 위한 데모로 사용되었습니다.

OAuth 인증 외에도 인증이 성공한 후 데이터베이스에 사용자를 생성하는 방법도 보여줍니다.

## 📝 Features

- **OAuth Authentication** with multiple providers:
  - <img src="https://github.com/Alfex4936/Alfex4936/assets/2356749/dc097b7b-2756-4cf1-a306-23130377dd46" width="20" height="20"> GitHub (깃헙)
  - <img src="https://github.com/Alfex4936/Alfex4936/assets/2356749/2c3ddcf4-36f7-40b6-b31e-832e608eb28e" width="20" height="20"> Google (구글)
  - <img src="https://github.com/Alfex4936/Alfex4936/assets/2356749/a2fd5b2a-aa0a-40fe-a0b8-3a5bc490e6c9" width="20" height="20"> Kakao (카카오)
  - <img src="https://github.com/Alfex4936/Alfex4936/assets/2356749/edafa506-8b43-4b61-ac60-551d369b6a15" width="20" height="20"> Naver (네이버)
  - :)


- **User Management**: Creating users in the database post-authentication.

- **Token Management**: JWT-based token issuance and validation for authenticated users.
<!-- 
## 📦 Project Structure

```
src/
|   main.rs
|
+---auth
|   |   mod.rs
|   |   token.rs
|   |   token_guard.rs
|   |
|   \---oauth
|       |   github_oauth.rs
|       |   google_oauth.rs
|       |   kakao_oauth.rs
|       |   mod.rs
|       |   naver_oauth.rs
|
+---config
|   |   env.rs
|   |   mod.rs
|
+---handlers
|   |   auth_handler.rs
|   |   mod.rs
|   |   oauth_handler.rs
|
+---models
|   |   app_state.rs
|   |   login_user_schema.rs
|   |   mod.rs
|   |   query_code.rs
|   |   register_user_schema.rs
|   |   token_claims.rs
|   |   user.rs
|
\---responses
    |   filtered_user.rs
    |   mod.rs
    |   user_response.rs
``` -->

## 🌱 Setup

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/Alfex4936/rust-actix-oauth2.git
   cd rust-actix-oauth2
   ```

2. **Setup Environment Variables**:

   Copy the sample `.env` content and set the appropriate values for your OAuth application:

   ```env
   CLIENT_ORIGIN=http://localhost:3001
   JWT_SECRET=your_secret
   ...
   # Fill other values accordingly.
   ```

3. **Run the Application**:

   ```bash
   cargo install cargo-watch
   
   cargo watch -q -c -w src/ -x run
   ```

   Hosted at `http://localhost:8080`.

## 🔍 Example: Naver OAuth2

To get an insight into how the OAuth flow works, here's a snippet for Naver OAuth:

```rust
// ... [snip] ...

pub async fn get_naver_oauth_token(
    authorization_code: &str,
    data: &web::Data<AppState>,
) -> Result<BasicOauthToken, Box<dyn Error>> {
    // ... [snip] ...
}

pub async fn get_naver_user(access_token: &str) -> Result<NaverUserResult, Box<dyn Error>> {
    // ... [snip] ...
}
```

## 🙌 Contributions

Let's make authentication easy for everyone!

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](MIT.md) file for details.

---