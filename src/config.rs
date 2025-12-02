use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_NAME: &'static str = "kanacheck.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path: String,
    pub characters: Vec<String>,
    pub extensions: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: "src".to_string(),
            characters: vec!["　".to_string()],
            extensions: vec!["js".to_string(), "ts".to_string(), "html".to_string()],
        }
    }
}
