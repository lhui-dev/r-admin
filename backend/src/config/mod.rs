use std::net::Ipv4Addr;

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub app: AppSettings,
    #[serde(default)]
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    #[serde(default)]
    pub log: LogSettings,
    #[serde(default)]
    pub jwt: JwtSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppSettings {
    #[serde(default = "default_app_name")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    #[serde(default = "default_server_host")]
    pub host: Ipv4Addr,
    #[serde(default = "default_server_port")]
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogSettings {
    #[serde(default = "default_log_level")]
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtSettings {
    #[serde(default = "default_jwt_secret")]
    pub secret: String,
    #[serde(default = "default_jwt_expires_in")]
    pub expires_in: u64,
}

impl Settings {
    pub fn load() -> anyhow::Result<Self> {
        let builder = config::Config::builder()
            .set_default("app.name", default_app_name())?
            .set_default("server.host", default_server_host().to_string())?
            .set_default("server.port", default_server_port())?
            .set_default("log.level", default_log_level())?
            .set_default("jwt.secret", default_jwt_secret())?
            .set_default("jwt.expires_in", default_jwt_expires_in())?
            .add_source(config::Environment::default().separator("_"));

        builder
            .build()
            .context("failed to build config")?
            .try_deserialize()
            .context("failed to deserialize config")
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            name: default_app_name(),
        }
    }
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            host: default_server_host(),
            port: default_server_port(),
        }
    }
}

impl Default for LogSettings {
    fn default() -> Self {
        Self {
            level: default_log_level(),
        }
    }
}

impl Default for JwtSettings {
    fn default() -> Self {
        Self {
            secret: default_jwt_secret(),
            expires_in: default_jwt_expires_in(),
        }
    }
}

fn default_app_name() -> String {
    "r-admin-backend".to_string()
}

fn default_server_host() -> Ipv4Addr {
    Ipv4Addr::new(0, 0, 0, 0)
}

fn default_server_port() -> u16 {
    8080
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_jwt_secret() -> String {
    // Local fallback only. Real deployments should always override this via
    // environment variables or a dedicated secrets manager.
    "change-me-for-production".to_string()
}

fn default_jwt_expires_in() -> u64 {
    7200
}
