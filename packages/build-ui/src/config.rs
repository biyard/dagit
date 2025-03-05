#![allow(dead_code)]
use dioxus_logger::tracing::Level;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub commit: &'static str,
    pub log_level: Level,
    pub api_url: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            commit: option_env!("COMMIT").unwrap_or("unknown"),
            log_level: match option_env!("RUST_LOG") {
                Some("trace") => Level::TRACE,
                Some("debug") => Level::DEBUG,
                Some("info") => Level::INFO,
                Some("warn") => Level::WARN,
                Some("error") => Level::ERROR,
                _ => Level::INFO,
            },
            api_url: option_env!("API_URL").unwrap_or("https://dagit-api.dev.biyard.co"),
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
        CONFIG.as_ref().unwrap()
    }
}
