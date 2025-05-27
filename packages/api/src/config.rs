use bdk::prelude::*;
use by_types::config::*;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub endpoint: &'static str,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
            endpoint: option_env!("API_URL").unwrap_or("http://localhost:3000"),
            port: option_env!("PORT")
                .unwrap_or("3000")
                .parse()
                .unwrap_or(3000),
        }
    }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        &CONFIG.as_ref().unwrap()
    }
}
