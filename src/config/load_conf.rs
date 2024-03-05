use core::panic;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Error, Read},
    path::Path,
};

use crate::algorithms;

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub id: i32,
    pub url: String,
    pub weight: i32,
    pub active: bool,
}

impl From<Server> for algorithms::utils::Server {
    fn from(server: Server) -> algorithms::utils::Server {
        algorithms::utils::Server {
            id: server.id,
            url: server.url,
            weight: server.weight,
            active: server.active,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub servers: Vec<Server>,
}

pub fn load(file_path: &str) -> Result<ConfigFile, Error> {
    let path = Path::new(file_path);
    let file = File::open(&path);

    if file.is_err() {
        let error = file.unwrap_err();
        println!(
            "Failed to load config from file '{}'\n[{}]: {}",
            file_path,
            error.kind(),
            error.to_string()
        );

        panic!();
    }

    let mut file = file.unwrap();
    let mut content = String::new();
    let read_content = file.read_to_string(&mut content);

    if read_content.is_err() {
        let error = read_content.unwrap_err();
        println!(
            "Failed to read config from file '{}'\n[{}]: {}",
            file_path,
            error.kind(),
            error.to_string()
        );

        panic!();
    }

    let config = match json5::from_str::<ConfigFile>(&content) {
        Ok(config) => config,
        Err(error) => {
            println!(
                "Failed to parse config from file: '{}'\n{}",
                file_path,
                error.to_string()
            );

            panic!();
        }
    };

    Ok(config)
}
