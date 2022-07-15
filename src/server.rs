use crate::response::{Header, HttpResponse};
use std::{
    fs,
    io::{prelude::*, ErrorKind},
    net::{TcpListener, TcpStream},
};

enum RequestError {
    Internal,
    MethodNotAllowed,
    BadRequest,
}

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

                            match req {
                                Ok(req) => {
                                    println!("[{}] {} {}", ip, req.0, req.1);

                                    self.handle_request(req, res);
                                }
                                Err(e) => {
                                    let body = String::new();
                                    match e {
                                        RequestError::MethodNotAllowed => res
                                            .status(405)
                                            .message("Method not allowed")
                                            .send_from_str(body, None),
                                        RequestError::BadRequest => res
                                            .status(400)
                                            .message("BadRequest")
                                            .send_from_str(body, None),

                                        RequestError::Internal => res
                                            .status(500)
                                            .message("Internal server error")
                                            .send_from_str(body, None),
                                    }
                                }
                            }
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

    fn get_req_data(&self, mut stream: &TcpStream) -> Result<(String, String), RequestError> {
        let mut buffer = [0; 1000];

        if let Err(e) = stream.read(&mut buffer) {
            eprintln!("Failed to process request.\nError:\n{}", e);
            return Err(RequestError::Internal);
        }

        let req_str = String::from_utf8(buffer.to_vec()).unwrap();
        if !req_str.starts_with("GET") {
            return Err(RequestError::MethodNotAllowed);
        }

        let req: Vec<&str> = req_str.split_whitespace().collect();

        if req.len() < 2 {
            return Err(RequestError::BadRequest);
        }
        let method = req[0];
        let url = req[1];

        return Ok((method.to_string(), url.to_string()));
    }

    fn send_not_found(&self, res: HttpResponse) {
        res.status(404)
            .message("Not found")
            .send_from_str("Resource not found".to_string(), None)
    }

    fn handle_request(&self, req: (String, String), mut res: HttpResponse) {
        let mut path = req.1;
        // Replace prefix with the actual path (if present at the start of the string)
        if path.starts_with(&self.prefix) {
            path = path.replacen(&self.prefix, &self.path, 1);
        }

        return match fs::read(&path) {
            Ok(contents) => {
                let mime = mime_guess::from_path(path);
                res.send_bytes(
                    contents,
                    Some(vec![(
                        "Content-Type".to_string(),
                        mime.first_or_text_plain().to_string(),
                    )]),
                );
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => self.send_not_found(res),
                e => {
                    res.status(500)
                        .message("Internal server error.")
                        .send_from_str("Failed to process the request.".to_string(), None);
                    println!("{e:#?}");
                }
            },
        };
    }
}
