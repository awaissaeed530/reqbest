use std::{net::TcpStream, io::{BufReader, BufRead, Read}, collections::HashMap};

struct Request {
    pub method: String,
    pub host: String,
    pub connection: String,
    pub headers: HashMap<String, String>
}

impl Default for Request {
    fn default() -> Self {
        Request {
            method: "".to_owned(),
            host: "".to_owned(),
            connection: "".to_owned(),
            headers: HashMap::new()
        }
    }
}

pub fn handle_client(stream: &TcpStream) {
    dbg!(&stream);
    read_request(&stream); 
}

fn read_request(stream: &TcpStream) {
    let mut request = Request::default();
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    let mut headers = String::new();
    loop {
        let bytes_read = reader.read_line(&mut headers).unwrap();
        if bytes_read < 3 {
            break;
        }
    }

    let mut lines = headers.lines();
    let method = lines.nth(0).unwrap();
    dbg!(method);

    for l in lines {
        let mut split = l.split(":");
        let key = split.nth(0).unwrap_or("").trim().to_owned();
        let value = split.nth(0).unwrap_or("").trim().to_owned();
        if key.is_empty() || value.is_empty() { continue; } 
        request.headers.insert(key, value); 
    }
    dbg!(&request.headers);

    let content_length = request.headers.get("Content-Length").unwrap().parse::<usize>().unwrap();
    let mut buf = vec![0; content_length];
    reader.read_exact(&mut buf).unwrap();

    let content = String::from_utf8(buf).unwrap();
    dbg!(content);
}

