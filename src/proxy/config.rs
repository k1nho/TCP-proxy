use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct App {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Ports")]
    pub ports: Vec<u16>,
    #[serde(rename = "Targets")]
    pub targets: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "Apps")]
    pub apps: Vec<App>,
}

/// Returns a configuration object to be used for a TCP proxy
///
/// **filename**: The json file to use to configure the proxy
///
/// # Panics:
///
/// Config will panic if it cannot find a json config file or if it cannot parse the json into
/// a config struct
pub fn config(filename: String) -> Config {
    // configure proxy based on json file
    let file_config_string = fs::read_to_string(filename);

    match file_config_string {
        Ok(file_content) => {
            let config = serde_json::from_str(&file_content)
                .expect("could not parse config file. Invalid json format");

            config
        }
        Err(_) => {
            panic!("Please provide config file to configure proxy");
        }
    }
}
