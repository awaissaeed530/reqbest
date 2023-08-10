use std::{net::{TcpListener, TcpStream}, io::Read};

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 128];
    let bytes_read = stream.read(&mut buf).unwrap();
    println!("{:?}", stream);
    println!("{:?}", bytes_read);
    println!("{:?}", String::from_utf8(buf.to_vec()).unwrap());
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
