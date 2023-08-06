#[derive(Debug, Clone)]
pub struct Config {
    pub client_origin: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: i64,
    // Google
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub google_oauth_redirect_url: String,
    // Github
    pub github_oauth_client_id: String,
    pub github_oauth_client_secret: String,
    pub github_oauth_redirect_url: String,
    // Naver
    pub naver_oauth_client_id: String,
    pub naver_oauth_client_secret: String,
    pub naver_oauth_redirect_url: String,
    // Kakao
    pub kakao_oauth_client_id: String,
    pub kakao_oauth_redirect_url: String,
}

impl Config {
    pub fn init() -> Config {
        let client_origin = std::env::var("CLIENT_ORIGIN").expect("CLIENT_ORIGIN must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in =
            std::env::var("TOKEN_EXPIRED_IN").expect("TOKEN_EXPIRED_IN must be set");
        let jwt_max_age = std::env::var("TOKEN_MAXAGE").expect("TOKEN_MAXAGE must be set");
        let google_oauth_client_id =
            std::env::var("GOOGLE_OAUTH_CLIENT_ID").expect("GOOGLE_OAUTH_CLIENT_ID must be set");
        let google_oauth_client_secret = std::env::var("GOOGLE_OAUTH_CLIENT_SECRET")
            .expect("GOOGLE_OAUTH_CLIENT_SECRET must be set");
        let google_oauth_redirect_url = std::env::var("GOOGLE_OAUTH_REDIRECT_URL")
            .expect("GOOGLE_OAUTH_REDIRECT_URL must be set");
        let github_oauth_client_id =
            std::env::var("GITHUB_OAUTH_CLIENT_ID").expect("GITHUB_OAUTH_CLIENT_ID must be set");
        let github_oauth_client_secret = std::env::var("GITHUB_OAUTH_CLIENT_SECRET")
            .expect("GITHUB_OAUTH_CLIENT_SECRET must be set");
        let github_oauth_redirect_url = std::env::var("GITHUB_OAUTH_REDIRECT_URL")
            .expect("GITHUB_OAUTH_REDIRECT_URL must be set");

        let naver_oauth_client_id =
            std::env::var("NAVER_OAUTH_CLIENT_ID").expect("NAVER_OAUTH_CLIENT_ID must be set");

        let naver_oauth_client_secret = std::env::var("NAVER_OAUTH_CLIENT_SECRET")
            .expect("NAVER_OAUTH_CLIENT_SECRET must be set");

        let naver_oauth_redirect_url = std::env::var("NAVER_OAUTH_REDIRECT_URL")
            .expect("NAVER_OAUTH_REDIRECT_URL must be set");

        // Kakao
        let kakao_oauth_client_id =
            std::env::var("KAKAO_OAUTH_CLIENT_ID").expect("KAKAO_OAUTH_CLIENT_ID must be set");

        let kakao_oauth_redirect_url = std::env::var("KAKAO_OAUTH_REDIRECT_URL")
            .expect("KAKAO_OAUTH_REDIRECT_URL must be set");

        Config {
            client_origin,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age: jwt_max_age.parse::<i64>().unwrap(),
            google_oauth_client_id,
            google_oauth_client_secret,
            google_oauth_redirect_url,
            github_oauth_client_id,
            github_oauth_client_secret,
            github_oauth_redirect_url,
            naver_oauth_client_id,
            naver_oauth_client_secret,
            naver_oauth_redirect_url,
            kakao_oauth_client_id,
            kakao_oauth_redirect_url,
        }
    }
}
