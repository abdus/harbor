use crate::{algorithms, config};
use once_cell::sync::Lazy;
use rand::Rng;
use std::{env, sync::Mutex};

#[derive(Debug)]
pub struct Server {
    pub id: i32,
    pub url: String,
    pub weight: i32,
    pub active: bool,
}

static mut SERVERS: Lazy<Mutex<Vec<Server>>> = Lazy::new(|| Mutex::new(vec![]));

impl Clone for Server {
    fn clone(&self) -> Server {
        Server {
            id: self.id,
            url: self.url.clone(),
            weight: self.weight,
            active: self.active,
        }
    }
}

pub fn load_servers_from_config() {
    let env = match env::var("HARBOR_CONFIG") {
        Ok(val) => val,
        Err(_) => "config.json".to_string(),
    };

    let config = config::load_conf::load(env.as_str());

    if config.is_err() {
        println!("Error: {:?}", config.err().unwrap());
        panic!();
    }

    let config = config.unwrap();
    let mut servers: Vec<Server> = Vec::new();

    for server in config.servers {
        let tmp: Server = server.into();
        servers.push(tmp);
    }

    unsafe {
        SERVERS.lock().unwrap().extend(servers);
    }
}

pub fn get_remote_server_address() -> String {
    let algo = "random"; // should be read from config

    match algo {
        "round_robin" => {
            let servers = unsafe { SERVERS.lock() };

            if servers.is_err() {
                println!("Error: {:?}", servers.err().unwrap());
                return "localhost:8081".to_string();
            }

            let servers = servers.unwrap();

            println!("{:#?}", servers);

            let servers = servers.to_vec();
            let servers = servers.iter().filter(|s| s.active).collect::<Vec<_>>();
            let server_length = servers.len() as i32;
            let remote_server_idx = algorithms::round_robin::round_robin(server_length);

            println!("{}", remote_server_idx);

            let server = servers.iter().find(|s| s.id == remote_server_idx).unwrap();
            let url = server.url.clone();

            println!("Selected server: {:#?}", url);

            url
        }
        "random" => {
            let servers = unsafe { SERVERS.lock() };

            if servers.is_err() {
                println!("Error: {:?}", servers.err().unwrap());
                return "localhost:8081".to_string();
            }

            let servers = servers.unwrap();
            let random_server_idx = rand::thread_rng().gen_range(0..servers.len());
            let server = servers.get(random_server_idx).unwrap();

            server.url.clone()
        }
        _ => "localhost:3001".to_string(),
    }
}
