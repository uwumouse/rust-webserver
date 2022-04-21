use crate::response::{Header, HttpResponse};
use std::{
    fs,
    io::{prelude::*, ErrorKind},
    net::{TcpListener, TcpStream},
};

pub struct HttpServer {
    headers: Vec<Header>,
    prefix: String,
    path: String,
}

impl HttpServer {
    pub fn new(prefix: String, path: String, headers: Vec<Header>) -> Self {
        Self {
            prefix,
            path,
            headers,
        }
    }

    pub fn listen(&self, port: u16) {
        let host = format!("127.0.0.1:{}", port.to_string());

        match TcpListener::bind(&host) {
            Ok(l) => {
                println!("Rust WebServer is setup and running on 127.0.0.1:{}", port);
                for stream in l.incoming() {
                    match stream {
                        Ok(stream) => {
                            let res = HttpResponse::new(&stream).headers(&mut self.headers.clone());
                            let req = self.get_req_data(&stream);
                            let ip = stream.peer_addr().unwrap().ip();

                            if let Err(e) = req {
                                println!("Request from {}:\n{}", ip, e);
                                res.status(500)
                                    .message("Internal server error")
                                    .send(e.to_string());
                                return;
                            }

                            let req = req.unwrap();

                            println!("[{}] {} {}", ip, req.0, req.1);

                            self.handle_request(req, res);
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    };
                }
            }
            Err(e) => {
                eprintln!("Failed to setup server on {}.\nError: {}", host, e);
                std::process::exit(1);
            }
        }
    }

    fn get_req_data(&self, mut stream: &TcpStream) -> Result<(String, String), String> {
        let mut buffer = [0; 1000];

        if let Err(e) = stream.read(&mut buffer) {
            return Err(format!("Failed to process request.\nError:\n{}", e));
        }

        let req_str = String::from_utf8(buffer.to_vec()).unwrap();
        let req: Vec<&str> = req_str.split_whitespace().collect();
        let method = req[0];
        let url = req[1];

        return Ok((method.to_string(), url.to_string()));
    }

    fn send_not_found(&self, res: HttpResponse) {
        res.status(404)
            .message("Not found")
            .send("Resource not found".to_string())
    }

    fn handle_request(&self, req: (String, String), mut res: HttpResponse) {
        let mut path = req.1;
        // Replace prefix with the actual path (if present at the start of the string)
        if path.starts_with(&self.prefix) {
            path = path.replace(&self.prefix, &self.path);
        }

        return match fs::read_to_string(path) {
            Ok(contents) => res.send(contents),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => self.send_not_found(res),
                _ => res
                    .status(500)
                    .message("Internal server error.")
                    .send("Failed to process the request.".to_string()),
            },
        };
    }
}
