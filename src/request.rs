use std::{net::TcpStream, io::{BufReader, BufRead, Read}, collections::HashMap};

#[derive(Debug)]
struct Request {
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Default for Request {
    fn default() -> Self {
        Request {
            method: "".to_owned(),
            headers: HashMap::new(),
            body: "".to_owned(),
        }
    }
}

pub fn handle_client(stream: &TcpStream) {
    dbg!(&stream);
    let request = read_request(&stream); 
    dbg!(&request);
}

fn read_request(stream: &TcpStream) -> Request {
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
    let method = lines.nth(0).unwrap().split("/").nth(0).unwrap().trim();
    request.method = method.to_owned();

    for l in lines {
        let mut split = l.split(":");
        let key = split.nth(0).unwrap_or("").trim().to_owned();
        let value = split.nth(0).unwrap_or("").trim().to_owned();
        if key.is_empty() || value.is_empty() { continue; } 
        request.headers.insert(key, value); 
    }

    let content_length = request.headers.get("Content-Length").unwrap().parse::<usize>().unwrap();
    let mut buf = vec![0; content_length];
    reader.read_exact(&mut buf).unwrap();

    let content = String::from_utf8(buf).unwrap();

    request.body = content;
    return request;
}

