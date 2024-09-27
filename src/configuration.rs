use std::env::{self, current_dir};

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Clone)]
pub struct BotSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub bot_id: u64,
    pub bot_token: String,
    pub main_chat_id: i64,
    pub main_chat_url: String,
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub bot: BotSettings,
}

const ENVIRONMENT_KEY: &'static str = "APP_ENVIRONMENT";

impl Settings {
    pub fn get_configuration() -> Result<Self, config::ConfigError> {
        let current_dir = current_dir().expect("failed to get current directory");
        let config_dir = current_dir.join("config");

        let environment: Environment = env::var(ENVIRONMENT_KEY)
            .unwrap_or("local".into())
            .try_into()
            .expect("failed to parse APP_ENVIRONMENT");

        let environment_filename = format!("{}.yaml", environment.as_str());

        let settings = config::Config::builder()
            .add_source(config::File::from(config_dir.join(environment_filename)))
            .build()?;

        settings.try_deserialize::<Settings>()
    }
}

enum Environment {
    Local,
    Production,
}

impl Environment {
    fn as_str(&self) -> &str {
        match self {
            Self::Local => "local",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            _ => Err(format!("Invalid environment: {value}")),
        }
    }
}
