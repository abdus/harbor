use crate::algorithms;
use once_cell::sync::Lazy;
use rand::Rng;
use std::sync::Mutex;

#[derive(Debug)]
struct Server {
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
    let servers = vec![
        Server {
            id: 0,
            url: "localhost:8081".to_string(),
            weight: 1,
            active: true,
        },
        Server {
            id: 1,
            url: "localhost:8082".to_string(),
            weight: 1,
            active: true,
        },
        Server {
            id: 2,
            url: "localhost:8083".to_string(),
            weight: 2,
            active: true,
        },
    ];

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
