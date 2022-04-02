use config::Config;
use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
  pub database: DatabaseSettings,
  pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
  pub port: u16,
  pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
  pub username: String,
  pub password: Secret<String>,
  pub port: u16,
  pub host: String,
  pub database_name: String,
}

pub enum Environment {
  Local,
  Production,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
  let environment: Environment = std::env::var("APP_ENVIRONMENT")
    .unwrap_or_else(|_| "local".into())
    .try_into()
    .expect("Failed to parse APP_ENVIRONMENT");

  let settings = Config::builder()
    .add_source(config::File::with_name("configuration/base"))
    .add_source(
      config::File::with_name(&format!("configuration/{}", environment.as_str())).required(true),
    )
    .build()
    .unwrap();

  settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
  pub fn connection_string(&self) -> Secret<String> {
    Secret::new(format!(
      "postgres://{}:{}@{}:{}/{}",
      self.username,
      self.password.expose_secret(),
      self.host,
      self.port,
      self.database_name
    ))
  }

  pub fn connection_string_without_db(&self) -> Secret<String> {
    Secret::new(format!(
      "postgres://{}:{}@{}:{}",
      self.username,
      self.password.expose_secret(),
      self.host,
      self.port
    ))
  }
}

impl Environment {
  pub fn as_str(&self) -> &'static str {
    match self {
      Environment::Local => "local",
      Environment::Production => "production",
    }
  }
}

impl TryFrom<String> for Environment {
  type Error = String;

  fn try_from(s: String) -> Result<Self, Self::Error> {
    match s.to_lowercase().as_str() {
      "local" => Ok(Self::Local),
      "production" => Ok(Self::Production),
      other => Err(format!(
        "{} is not a supported environment. Use either `local` or `production`.",
        other
      )),
    }
  }
}
