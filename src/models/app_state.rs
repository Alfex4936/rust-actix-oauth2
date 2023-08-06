use crate::config::env::Config;
use crate::models::user::User;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub db: Arc<Mutex<Vec<User>>>,
    pub env: Config,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            db: Arc::new(Mutex::new(Vec::new())),
            env: Config::init(),
        }
    }
}
