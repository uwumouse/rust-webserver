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

    fn build(&self, body: String) -> String {
        let mut res = format!("HTTP/1.1 {} {}\n", self.status, self.message);

        for h in &self.headers {
            res.push_str(&format!("{}: {}\n", h.0, h.1));
        }

        res.push_str(&format!("\n{}", body));

        res
    }

    pub fn send(&mut self, body: String) {
        let res = self.build(body);

        self.stream.write_all(res.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}
