mod request;

use std::net::TcpListener;

use request::handle_client;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        handle_client(&stream.unwrap());
    }
}
