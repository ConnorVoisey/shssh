use serde::Deserialize;
use std::{env, fs::File};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub servers: Vec<Server>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub label: String,
    pub key: String,
    pub host: String,
    pub users: Vec<User>,
    pub tags: Option<Vec<String>>,
    pub databases: Option<Vec<Database>>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub name: String,
    pub password: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    pub name: String,
    pub user: String,
    pub password: String,
    pub db_type: Option<String>,
}

pub fn get_config() -> Option<Config> {
    let file_path = get_config_file_path();
    let json_file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!(
                "Could not open config file at: {}. Make sure that the file exists and is readable",
                &file_path
            );
            eprintln!("You can set the config file path with the env variable: $SHSSH_CONFIG. Else this will default to $HOME/.config/shssh.json");
            eprintln!("error: {err}");
            return None;
        }
    };
    serde_json::from_reader(json_file).expect("failed to parse json")
}
fn get_config_file_path() -> String {
    match env::var("SHSSH_CONFIG") {
        Ok(val) => val,
        Err(_) => {
            let home_os = env::var("HOME").expect("Enviroment Variable 'HOME' is required");
            format!("{}/.config/shssh.json", home_os.as_str())
        }
    }
}
