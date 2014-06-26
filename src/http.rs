use easy::{Curl};
use info;
use libc;
use opt;
use std::collections::HashMap;
use std::{io, mem, str, vec};

pub enum Method {
    GET,
    POST,
    DELETE,
    HEAD,
    PUT,
}

#[deriving(Show)]
pub enum Error {
    InvalidUrl,
    NetworkError,
}

pub struct Client {
    base_url: String,
    session: Curl,
}

pub trait ContentWrapper {
    fn as_bytes<'a>(&'a self) -> &'a [u8];
}

pub struct Response {
    url: String,
    headers: HashMap<String, String>,
    status_code: u8,
    status_message: String,
    content: Option<Box<ContentWrapper>>
}

pub struct Request {
    url: String,
    pub method: Method,
    headers: HashMap<String, String>,
}

struct Dumb<'a> {
    handler: &'a mut io::Writer,
}

impl Client {
    pub fn new(base_url: &str) -> Client {
        Client {
            base_url: base_url.to_string(),
            session: Curl::new(),
        }
    }

    fn get_rel_url(base_url: &str, rel_url: &str) -> String {
        // FIXME: do a correct transition
        let mut res = base_url.to_string();
        res.push_str("/");
        res.push_str(rel_url);
        res
    }

    fn new_method_request(&self, rel_url: &str, method: Method) -> Request {
        Request {
            url: Client::get_rel_url(self.base_url.as_slice(), rel_url),
            method: method,
            headers: HashMap::new()
        }
    }

    pub fn new_get_request(&self, rel_url: &str) -> Request {
        self.new_method_request(rel_url, GET)
    }

    pub fn new_post_request(&self, rel_url: &str) -> Request {
        self.new_method_request(rel_url, POST)
    }

    fn http_write_fn(p: *u8, size: libc::size_t, nmemb: libc::size_t, user_data: *libc::c_void) -> libc::size_t {
        let dumb: *Dumb = unsafe { mem::transmute(user_data) };
        let buf = unsafe { vec::raw::from_buf(p, (size * nmemb) as uint) };
        match unsafe { (*dumb).handler.write(buf.as_slice())} {
            Ok(_) => (size * nmemb),
            _ => 0
        }
    }

    pub fn perform(&mut self, req: &Request) -> Result<Response, Error> {
        self.session.setopt(opt::URL, req.url.as_slice());
        let header_vec: Vec<String> = req.headers.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
        self.session.setopt(opt::HTTPHEADER, header_vec);
        self.session.setopt(opt::VERBOSE, false);

        let mut response = Response {
            status_code: 0,
            url: "".to_string(),
            headers: HashMap::new(),
            status_message: "".to_string(),
            content: None
        };

        let mut handler = io::MemWriter::new();
        let res = {
            let dumb = Dumb {
                handler: &mut handler,
            };
            let dumb_ptr: *Dumb = &dumb;

            self.session.set_write_func(Client::http_write_fn);
            self.session.setopt(opt::WRITEDATA, dumb_ptr);
            self.session.perform()
        };

        let res = match res {
            0 => {
                let mut val : Option<int> = self.session.getinfo(info::RESPONSE_CODE);
                response.status_code = val.unwrap() as u8;
                response.content = Some(box handler as Box<ContentWrapper>);
                Ok(response)
            },
            _ => {
                Err(NetworkError)
            }
        };

        res
    }
}

impl Request {
    pub fn new() -> Request {
        Request {
            url: "".to_string(),
            method: GET,
            headers: HashMap::new()
        }
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_string(), value.to_string());
    }
}

impl ContentWrapper for io::MemWriter {
    fn as_bytes<'a>(&'a self) -> &'a [u8] {
        self.get_ref()
    }
}

#[cfg(test)]
mod test
{
    use super::{Client, Request, Response};
    use std::str;

    #[test]
    fn simple_get() {
        let mut c = Client::new("http://baidu.com");
        let mut req = c.new_get_request("/");
        req.set_header("User-Agent", "CRust/0.0.1");

        let resp = c.perform(&req).unwrap();
        assert_eq!(resp.status_code, 200);
        let content = resp.content.unwrap();
        let content_bytes = content.as_bytes();
        let text = unsafe { str::raw::from_utf8(content_bytes) };
        assert!(text.find_str("www.baidu.com").is_some());
    }
}
