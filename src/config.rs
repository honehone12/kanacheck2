use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_NAME: &'static str = "kanacheck.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub characters: Vec<String>,
    pub extensions: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            characters: vec!["　".to_string()],
            extensions: vec!["js".to_string(), "ts".to_string(), "html".to_string()],
        }
    }
}
