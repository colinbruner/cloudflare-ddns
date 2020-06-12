use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    //pub key: String,
    //pub email: String,
    pub zone: String,
    pub zone_id: String,
    pub token: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(".cloudflare-ddns"))?;
        s.try_into()
    }
}
