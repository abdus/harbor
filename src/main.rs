mod algorithms;

use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use std::thread;
use std::{io::Write, net::TcpListener};

fn main() {
    // load remote servers from config
    algorithms::utils::load_servers_from_config();

    tcp_server();
}

fn header_parser(req_headers: &Vec<u8>) -> (HashMap<String, String>, bool) {
    let mut store = [httparse::EMPTY_HEADER; 100];
    let mut parser = httparse::Request::new(&mut store);
    let bytes_payload = &req_headers[..];
    let parsed = parser.parse(&bytes_payload).unwrap();
    let mut headers = HashMap::new();

    for header in parser.headers {
        let length = header.value.len();
        let value = String::from_utf8_lossy(header.value);

        if length > 0 {
            headers.insert(header.name.to_string(), value.to_string());
        }
    }

    (headers, parsed.is_complete())
}

fn tcp_server() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    println!("Server started at port 8080!");

    for stream in listener.incoming() {
        thread::spawn(|| {
            if stream.is_ok() {
                let mut stream = stream.unwrap();

                let mut data: Vec<u8> = Vec::new();
                let mut buffer = vec![0; 10];

                loop {
                    let headers = header_parser(&data).0;
                    let is_header_parsed = header_parser(&data).1;
                    let content_length_val = headers.get("Content-Length");

                    let content_length_val = if content_length_val.is_some() {
                        Some(content_length_val.unwrap().to_string())
                    } else if is_header_parsed {
                        Some("0".to_string())
                    } else {
                        None
                    };

                    //println!("Headers: {:#?}", headers);

                    if content_length_val.is_some() {
                        let content_length = content_length_val.unwrap().parse::<usize>().unwrap();

                        let raw_headers = String::from_utf8_lossy(&data).to_string();
                        let raw_headers = raw_headers.split("\r\n\r\n").collect::<Vec<&str>>();
                        let raw_headers = raw_headers.first();

                        if raw_headers.is_some() {
                            let headers = raw_headers.unwrap();
                            let payload_len = headers.len() + content_length;

                            if data.len() >= payload_len {
                                break;
                            }
                        }
                    }

                    let buffer_len = stream.read(&mut buffer).unwrap_or_default();
                    data.extend_from_slice(&buffer[..buffer_len]);

                    if buffer_len == 0 {
                        break;
                    }
                }

                let request_payload = data;
                let remote_server_addr = algorithms::utils::get_remote_server_address();
                let remote_req = TcpStream::connect(remote_server_addr);

                if remote_req.is_ok() {
                    let mut remote_stream = remote_req.unwrap();

                    remote_stream.write_all(&request_payload).unwrap();

                    let mut remote_data_buffer = vec![0; 10];

                    loop {
                        let buffer_len = remote_stream.read(&mut remote_data_buffer).unwrap();

                        stream.write_all(&remote_data_buffer[..buffer_len]).unwrap();
                        stream.flush().unwrap();

                        if buffer_len == 0 {
                            stream.shutdown(std::net::Shutdown::Both).unwrap();
                            break;
                        }
                    }
                } else {
                    stream
                        .write_all(&prepare_response(
                            b"Failed to connect to remote server".to_vec(),
                        ))
                        .unwrap();
                }
            }
        });
    }
}

fn prepare_response(body: Vec<u8>) -> Vec<u8> {
    let mut resp = Vec::new();

    resp.extend_from_slice(b"HTTP/1.1 200 OK\r\n");
    resp.extend_from_slice(b"Content-Length: ");
    resp.extend_from_slice(body.len().to_string().as_bytes());

    resp.extend_from_slice(b"\r\n\r\n");
    resp.extend_from_slice(&body);

    resp
}
