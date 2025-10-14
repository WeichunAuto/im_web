use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::Path};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server_config: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    /// load application host and port.
    pub fn load() -> Result<Self> {
        let host = env::var("APP_HOST").ok();
        let port = env::var("APP_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok()); // convert String into u16

        // 1. First Priority, use values if provided by the env.
        if let (Some(h), Some(p)) = (&host, port) {
            return Ok(AppConfig {
                server_config: ServerConfig {
                    host: h.to_string(),
                    port: p,
                },
            });
        }

        // 2. Second Priority, use values if application.yml file is provided.
        let path = Path::new("application.yml");
        if path.exists() {
            let file = File::open("application.yml").expect("无法解析");

            let mut app_config: AppConfig = serde_yaml::from_reader(file).expect("解析失败");

            if let Some(h) = host {
                app_config.server_config.host = h;
            }

            if let Some(p) = port {
                app_config.server_config.port = p;
            }
            Ok(app_config)
        } else {
            // 3. Third Priority, use default if both env and config file were not provided.
            println!("Can not find config file application.yml!");
            Ok(AppConfig {
                server_config: ServerConfig {
                    host: host.unwrap_or("127.0.0.1".to_string()),
                    port: port.unwrap_or(8080),
                },
            })
        }
    }
}
