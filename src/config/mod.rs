use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::Path};

#[derive(Debug, Deserialize, Serialize)]
struct JsonConfig {
    server: Option<ServerConfig>,
    database: Option<DatabaseConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_host: String,
    pub db_port: u16,
}

impl Config {
    pub fn new() -> Self {
        if !Path::new("./config.json").exists() {
            let mut file = File::create("./config.json").expect("Could not create config.json");

            let config = JsonConfig {
                server: Some(ServerConfig {
                    host: "127.0.0.1".to_string(),
                    port: 3000,
                }),
                database: Some(DatabaseConfig {
                    host: "127.0.0.1".to_string(),
                    port: 5432,
                    user: "".to_string(),
                    password: "".to_string(),
                    database: "".to_string(),
                }),
            };

            file.write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes())
                .expect("Could not write to config.json");

            panic!("Could not find config.json so i created one :)");
        }

        let config = std::fs::read_to_string("./config.json");

        if config.is_err() {
            panic!("Could not read config.json");
        }

        let config = config.unwrap();

        let json_config: JsonConfig = serde_json::from_str(&config).unwrap_or_else(|_| {
            panic!("Could not parse config.json");
        });

        let (db_host, db_port, db_password, db_user, db_database): (
            String,
            u16,
            String,
            String,
            String,
        ) = match json_config.database {
            Some(db) => {
                let host: String = db.host;
                let port: u16 = db.port;
                let password: String = db.password;
                let user: String = db.user;
                let database: String = db.database;

                (host, port, password, user, database)
            }
            None => {
                panic!("Could not find database config");
            }
        };

        let (server_host, server_port): (String, u16) = match json_config.server {
            Some(server) => {
                let host: String = server.host;
                let port: u16 = server.port;

                (host, port)
            }
            None => {
                panic!("Could not find server config");
            }
        };

        Config {
            port: server_port,
            host: server_host,
            db_user,
            db_password,
            db_name: db_database,
            db_host,
            db_port,
        }
    }
}
