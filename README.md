# harbor

Harbor is a lightweight load balancer written in Rust. It is fun side-project
to learn more about networking and load balancing. And also to learn Rust
better. Not intended for production use.

This project does not use any external libraries for handling HTTP requests. It
is built from scratch using the `std::net` and `std::thread` modules. I used
`httparse` for parsing the HTTP requests.

## Roadmap

Here are some of the features that I plan to implement in the future. As of
now, I am focusing on the basic features and will add more features as I go
along.

- [x] Supports HTTP/1.1
- [ ] Add support for HTTP/2
- [ ] Add support for `keep-alive` connections
- Load balancing algorithms
  - [x] Round-robin
  - [ ] Weighted round-robin
  - [ ] Least Active Connections
  - [x] Random
  - [ ] Least Response Time
  - [ ] IP Hash
- [ ] Health checks for backend servers
- [ ] Configuration file support
- [ ] Logging

## Usage

First of all, you will need to have Rust installed on your system. Then you
could simply run the `./start.sh` script to start the server. The server will
start on `localhost:8080` by default. This will also start a few backend
servers on `localhost:8081`, `localhost:8082` and `localhost:8083`.

You can pass a config file. Use environment variable `HARBOR_CONFIG` to specify
the path to the config file. You can find an example config in the
[config.json](./config.json) file.

To test the load balancer performance (and how it performs under load), you can
use the `siege` tool. Here is an example command to test the load balancer:

```sh
siege -t1M -c1000 -v http://0.0.0.0:8080

# -t1M: Run the test for 1 minute
# -c1000: Use 1000 concurrent connections
# -v: Verbose output
```

And this is how it performed on my machine (Ryzen 5/16GB RAM):

```txt
Lifting the server siege...
Transactions:		      173012 hits
Availability:		      100.00 %
Elapsed time:		       60.58 secs
Data transferred:	       25.29 MB
Response time:		        0.31 secs
Transaction rate:	     2855.93 trans/sec
Throughput:		        0.42 MB/sec
Concurrency:		      892.15
Successful transactions:      173013
Failed transactions:	           0
Longest transaction:	       54.73
Shortest transaction:	        0.00
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details.

Check out [Coding Challenges](https://codingchallenges.fyi/challenges/challenge-load-balancer).
