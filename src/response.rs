use std::{io::Write, net::TcpStream};

pub type Header = (String, String);

pub struct HttpResponse<'a> {
    status: u16,
    message: &'static str,
    headers: Vec<Header>,
    stream: &'a TcpStream,
}
impl<'a> HttpResponse<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        Self {
            status: 200,
            message: "OK",
            headers: Vec::new(),
            stream,
        }
    }

    pub fn message(mut self, message: &'static str) -> Self {
        self.message = message;
        self
    }

    pub fn status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }

    pub fn headers(mut self, appended: &mut Vec<Header>) -> Self {
        self.headers.append(appended);

        self
    }

    fn build_response(
        &self,
        mut body: Vec<u8>,
        additional_headers: Option<Vec<(String, String)>>,
    ) -> Vec<u8> {
        let mut res = format!("HTTP/1.1 {} {}\n", self.status, self.message)
            .as_bytes()
            .to_vec();

        HttpResponse::append_headers(&mut res, &self.headers);
        if let Some(h) = additional_headers {
            HttpResponse::append_headers(&mut res, &h);
        }

        res.append(&mut "\n".as_bytes().to_vec());
        res.append(&mut body);

        res
    }

    fn append_headers(res: &mut Vec<u8>, headers: &Vec<(String, String)>) {
        for h in headers {
            res.append(&mut format!("{}: {}\n", h.0, h.1).as_bytes().to_vec());
        }
    }

    pub fn send_from_str(&mut self, body: String, headers: Option<Vec<(String, String)>>) {
        self.send_bytes(body.as_bytes().to_vec(), headers);
    }

    pub fn send_bytes(&mut self, body: Vec<u8>, headers: Option<Vec<(String, String)>>) {
        let res = self.build_response(body, headers);

        self.stream.write_all(&res).unwrap();
        self.stream.flush().unwrap();
    }
}
