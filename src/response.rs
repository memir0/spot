use std::collections::HashMap;

pub struct Response {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new(body: String, headers: HashMap<String, String>) -> Response {
        return Response {
            status: 404,
            body: body,
            headers: headers,
        };
    }

    pub fn status(&mut self, new_status: u16) {
        self.status = new_status;
    }

    pub fn header(&mut self, name: String, value: String) {
        if self.headers.contains_key(&name) {
            self.headers.remove(&name);
        }
        self.headers.insert(name, value);
    }

    pub fn body(&mut self, text: impl AsRef<str>) {
        self.body = text.as_ref().to_string();
        self.header(String::from("content-length"), self.body.len().to_string());
        self.header(
            String::from("content-type"),
            String::from("text/html; charset=UTF-8"),
        );
    }

    pub fn to_http(self) -> String {
        let mut http = String::from(format!("HTTP/1.1 {} \n", self.status));
        for (key, value) in self.headers {
            http.push_str(&format!("{}: {}\n", key, value)[..]);
        }
        http.push('\n');
        http.push_str(&self.body[..]);
        return http;
    }
}
