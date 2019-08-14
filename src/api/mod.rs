pub mod search;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn new(key: &str) -> ApiKey {
        ApiKey(key.into())
    }
}
