use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: String,
}
