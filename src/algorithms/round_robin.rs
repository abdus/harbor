use std::sync::Mutex;

use once_cell::sync::Lazy;

static CURRENT_SERVER_IDX: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

pub fn round_robin(server_count: i32) -> i32 {
    let mut current_server_idx = CURRENT_SERVER_IDX.lock().unwrap();

    if *current_server_idx >= server_count {
        *current_server_idx = 0;
    }

    let server = *current_server_idx;

    *current_server_idx += 1;

    server
}
