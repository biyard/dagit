use bdk::prelude::dioxus_logger::tracing::Level;
use dioxus_oauth::firebase::FirebaseConfig;
#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub commit: &'static str,
    pub log_level: Level,
    pub api_url: &'static str,
    pub api_endpoint: &'static str,
    pub base_url: &'static str,
    pub firebase: FirebaseConfig,
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
            base_url: option_env!("BASE_URL").unwrap_or("https://dev.dagit.club"),
            api_url: option_env!("API_ENDPOINT").unwrap_or("https://api.dev.dagit.club"),
            api_endpoint: option_env!("API_ENDPOINT").unwrap_or("https://api.dev.biyard.co"),
            firebase: FirebaseConfig {
                api_key: option_env!("FIREBASE_API_KEY")
                    .expect("You must set FIREBASE_API_KEY")
                    .to_string(),
                auth_domain: option_env!("FIREBASE_AUTH_DOMAIN")
                    .expect("You must set FIREBASE_AUTH_DOMAIN")
                    .to_string(),
                project_id: option_env!("FIREBASE_PROJECT_ID")
                    .expect("You must set FIREBASE_PROJECT_ID")
                    .to_string(),
                storage_bucket: option_env!("FIREBASE_STORAGE_BUCKET")
                    .expect("You must set FIREBASE_STORAGE_BUCKET")
                    .to_string(),
                messaging_sender_id: option_env!("FIREBASE_MESSAGING_SENDER_ID")
                    .expect("You must set FIREBASE_MESSAGING_SENDER_ID")
                    .to_string(),
                app_id: option_env!("FIREBASE_APP_ID")
                    .expect("You must set FIREBASE_APP_ID")
                    .to_string(),
                measurement_id: option_env!("FIREBASE_MEASUREMENT_ID")
                    .expect("You must set FIREBASE_MEASUREMENT_ID")
                    .to_string(),
            },
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
