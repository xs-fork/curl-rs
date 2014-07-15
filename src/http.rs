use easy::{Curl};
use info;
use libc;
use opt;
use errors::CURLE_OK;
use std::collections::HashMap;
use std::io::{MemWriter, MemReader};
use std::{c_str, c_vec, mem, ptr};

pub static CURL_ERROR_SIZE: uint = 256;

/// HTTP Method, nuff said
///
/// Custom uses a static str as it is hard
/// to believe that anything else might happen,
/// i.e. that method name will be generated
/// dynamically
pub enum Method {
    Get,
    Post,
    Delete,
    Head,
    Put,
    Custom(&'static str)
}

#[deriving(Show)]
struct CurlError {
    pub code: uint,
    pub message: String
}

type ResponseWriteClosure<'a> = |&c_vec::CVec<u8>|:'a -> uint;

/// The general HTTP client which is tied to a specific
/// base URL and allows easy construction of relative requests
///
/// Cookies are automatically shared between all requests
pub struct Client {
    base_url: String,
    session: Curl,
    error_buf: Vec<u8>
}

/// Represents HTTP response
pub struct Response {
    pub url: String,
    pub headers: HashMap<String, String>,
    pub status_code: u16,
    pub status_message: String,
    pub content_data: Option<Box<Reader>>,
}

trait PairedWriter: Writer {
    fn consume(w: Self) -> Box<Reader>;
}

impl PairedWriter for MemWriter {
    fn consume(w: MemWriter) -> Box<Reader> {
        let buf = w.unwrap();
        box MemReader::new(buf) as Box<Reader>
    }
}

/// Represents HTTP request
pub struct Request {
    url: String,

    pub headers: HashMap<String, String>,
    pub method: Method,
    pub follow_redirects: bool,

    /// Transfer timeout in seconds
    pub timeout: Option<uint>,
    /// Connection timeout in seconds
    pub connection_timeout: Option<uint>,
}

impl Client {
    /// Constructs a new client with a base URL
    pub fn new(base_url: &str) -> Client {
        let session = Curl::new();
        session.setopt(opt::NOSIGNAL, true);

        let mut error_buf = Vec::with_capacity(CURL_ERROR_SIZE + 1);

        // Setup all default functions at once
        session.set_data_func(opt::WRITEFUNCTION, Client::http_write_fn);
        session.set_data_func(opt::HEADERFUNCTION, Client::http_header_fn);
        session.set_data_func(opt::READFUNCTION, Client::http_read_fn);
        session.set_progress_func(Client::http_progress_fn);

        // FIXME: check on practice if ERRORBUFFER provides
        // more verbosity than just using strerror as the latter
        // one sounds easier
        session.setopt(opt::ERRORBUFFER, error_buf.as_mut_ptr());

        Client {
            base_url: base_url.to_string(),
            session: session,
            error_buf: error_buf,
        }
    }

    fn get_rel_url(base_url: &str, rel_url: &str) -> String {
        // FIXME: do a correct transition
        let mut res = base_url.to_string();
        res.push_str("/");
        res.push_str(rel_url);
        res
    }

    /// Constructs GET request relatively to base URL
    pub fn new_get_request(&self, rel_url: &str) -> Request {
        // FIXME: redundand string duplication
        Request::new(Client::get_rel_url(self.base_url.as_slice(), rel_url).as_slice(), Get)
    }

    /// Constructs POST request relatively to base URL
    pub fn new_post_request(&self, rel_url: &str) -> Request {
        // FIXME: redundand string duplication
        Request::new(Client::get_rel_url(self.base_url.as_slice(), rel_url).as_slice(), Post)
    }

    fn update_for_method(&mut self, method: Method) -> int {
        match method {
            Get => self.session.setopt(opt::HTTPGET, true),
            Post => self.session.setopt(opt::HTTPPOST, true),
            Put => self.session.setopt(opt::UPLOAD, "PUT"),
            Delete => self.session.setopt(opt::CUSTOMREQUEST, "DELETE"),
            Custom(name) => self.session.setopt(opt::CUSTOMREQUEST, name),
            Head => {
                let res = self.session.setopt(opt::HTTPGET, true);
                if res != 0 {
                    res
                }
                else {
                    self.session.setopt(opt::NOBODY, true)
                }
            },
        }
    }

    // FIXME: for PUT set READDATA, INFILESIZE, INFILESIZE_LARGE
    // as described here: http://curl.haxx.se/libcurl/c/CURLOPT_UPLOAD.html
    // for POST set POSTFIELDS, POSTFIELDSIZE, POSTFIELDSIZE_LARGE
    // as described here: http://curl.haxx.se/libcurl/c/CURLOPT_POST.html
    // for PUT/POST provide READFUNCTION
    /// Sends request to server and returns a response (if any)
    pub fn perform(&mut self, req: &Request) -> Result<Response, CurlError> {
        let _ = self.session.setopt(opt::URL, req.url.as_slice());
        let _ = self.session.setopt(opt::USERAGENT, "CRust/0.0.1");

        if req.headers.len() > 0 {
            let header_vec: Vec<String> = req.headers.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
            self.session.setopt(opt::HTTPHEADER, header_vec);
        };
        let _ = self.session.setopt(opt::VERBOSE, false);
        let _ = self.update_for_method(req.method);
        let _ = self.session.setopt(opt::FOLLOWLOCATION, req.follow_redirects);

        // FIXME: introduce another option?
        let _ = self.session.setopt(opt::AUTOREFERER, req.follow_redirects);

        if req.timeout.is_some() {
            self.session.setopt(opt::TIMEOUT, req.timeout.unwrap() as int);
        }

        if req.connection_timeout.is_some() {
            // FIXME: check if it will work correctly with NOSIGNAL
            self.session.setopt(opt::CONNECTTIMEOUT, req.connection_timeout.unwrap());
        }

        let mut writer = MemWriter::new();

        let mut response = Response {
            status_code: 0,
            url: "".to_string(),
            headers: HashMap::new(),
            status_message: "".to_string(),
            content_data: None
        };

        let res = {
            let ptr = unsafe {response.as_ptr()};
            self.session.setopt(opt::HEADERDATA, ptr);

            let mut cl: ResponseWriteClosure = |buf| {
                match writer.write(buf.as_slice()) {
                    Ok(_) => buf.len(),
                    _ => 0
                }
            };
            self.session.setopt(opt::WRITEDATA, &mut cl as *mut ResponseWriteClosure);

            self.session.perform()
        };

        // Cleanup any unsafe data which is bound to current request
        self.session.setopt(opt::HEADERDATA, 0u);
        self.session.setopt(opt::WRITEDATA, 0u);

        let res = match (res as libc::c_uint) {
            CURLE_OK => {
                let val: Option<int> = self.session.getinfo(info::RESPONSE_CODE);
                response.status_code = val.unwrap() as u16;

                let val: Option<String> = self.session.getinfo(info::EFFECTIVE_URL);
                response.url = val.unwrap();

                let reader = PairedWriter::consume(writer);
                response.content_data = Some(reader);

                Ok(response)
            },
            _ => Err(CurlError {
                code: res,
                message: self.error_buf.as_slice().to_c_str().as_str().unwrap().to_string()
            }),
        };

        res
    }

    // Curl callbacks implementations
    // Write expects user_data to be ptr to closure f: |&CVec<u8>| -> uint
    // which should write that buffer anywhere it wants
    #[allow(unused_variable)]
    fn http_write_fn(p: *mut u8, size: libc::size_t, nmemb: libc::size_t,
                     user_data: *mut libc::c_void) -> libc::size_t {
        let resp: *mut ResponseWriteClosure = unsafe { mem::transmute(user_data) };
        if resp == ptr::mut_null() {
            size * nmemb
        } else {
            unsafe {
                let buf = c_vec::CVec::new(p, (size * nmemb) as uint);
                (*resp)(&buf) as libc::size_t
            }
        }
    }

    // Read expects user_data to be *Request
    #[allow(unused_variable)]
    fn http_read_fn(p: *mut u8, size: libc::size_t, nmemb: libc::size_t,
                    user_data: *mut libc::c_void) -> libc::size_t {
        size * nmemb
    }

    // Header expects user_data to be *Response
    fn http_header_fn(p: *mut u8, size: libc::size_t, nmemb: libc::size_t,
                      user_data: *mut libc::c_void) -> libc::size_t {
        let response: *mut Response = unsafe { mem::transmute(user_data) };
        if response == ptr::mut_null() {
            size * nmemb
        } else {
            // FIXME: huge unsafe block
            unsafe {
                let value: &str = mem::transmute(c_vec::CVec::new(p, (size * nmemb) as uint).as_slice());
                let re = regex!(r"HTTP/\d\.\d \d{3} (.*)");

                // FIXME: it could be done faster with falling back
                // to regex only if there is definitely HTTP/ prefix
                match re.captures(value) {
                    Some(caps) => {
                        // if it looks like HTTP status string
                        // all prev headers should be dropped
                        // and status message set again
                        (*response).status_message = caps.at(1).to_string();
                        (*response).headers.clear();
                    },
                    _ => {
                        // FIXME: trailers?
                        // Simple header processing
                        // Magic constants here:
                        // len - 4 -> len - 1 is the last index and there should be at
                        //            least 3 more symbols: ": " and at least one for value
                        // pos + 2 -> skip ": "
                        // FIXME: check actual HTTP specs
                        match value.find(':') {
                            Some(pos) if pos < value.len() - 4 => {
                                let name = value.slice_to(pos).to_string();
                                let value = value.slice_from(pos + 2).to_string();
                                (*response).headers.insert(name, value);
                            },
                            _ => debug!("Check out this header value: {}", value)
                        }
                    }
                }
            }

            size * nmemb
        }
    }

    // Progress expects user_data to be *Response
    #[allow(unused_variable)]
    fn http_progress_fn(user_data: libc::uintptr_t, dltotal: libc::c_double,
                        dlnow: libc::c_double, ultotal: libc::c_double,
                        ulnow: libc::c_double) -> libc::size_t {
        1
    }
}

impl Request {
    pub fn new(url: &str, method: Method) -> Request {
        Request {
            url: url.to_string(),
            method: method,
            headers: HashMap::new(),
            follow_redirects: false,
            timeout: None,
            connection_timeout: Some(0),
        }
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_string(), value.to_string());
    }
}

impl Response {
    pub unsafe fn as_ptr(&self) -> *const Response {
        mem::transmute(self)
    }
}

#[cfg(test)]
mod test
{
    use super::{Client};

    #[test]
    fn simple_get() {
        let mut c = Client::new("http://baidu.com");
        let req = c.new_get_request("/");

        let resp = c.perform(&req).unwrap();
        assert_eq!(resp.status_code, 200);
        let content = resp.content_data.unwrap().read_to_string().unwrap();
        assert!(content.as_slice().find_str("www.baidu.com").is_some());
    }

    #[test]
    fn redirections() {
        let mut c = Client::new("http://google.com");
        let mut req = c.new_get_request("/");

        let resp = c.perform(&req).unwrap();
        assert_eq!(resp.status_code / 100, 3);

        req.follow_redirects = true;
        let resp = c.perform(&req).unwrap();
        assert_eq!(resp.status_code, 200);
    }

    #[test]
    fn headers() {
        let mut c = Client::new("http://google.com");
        let mut req = c.new_get_request("/");
        req.follow_redirects = true;

        let resp = c.perform(&req).unwrap();
        assert_eq!(resp.status_code, 200);
        assert!(resp.headers.len() > 0);

        let ct = resp.headers.find_equiv(&"Content-Type").unwrap();
        assert!(ct.as_slice().starts_with("text/html"));
    }
}
