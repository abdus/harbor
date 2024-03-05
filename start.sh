#!/bin/bash

set -e;

clear;

start_server() {
  cd ./dummy/$1 && python -m http.server $1 &
}

start_load_balancer() {
  cargo run --release --bin harbor
}

# start the proxy server
start_server 8081
start_server 8082
start_server 8083

# start the load balancer
start_load_balancer

wait;
wait;
wait;


# optionally, test the load balancer with curl
# curl --parallel --parallel-immediate --parallel-max 3 --config dummy/curl-urls.txt
