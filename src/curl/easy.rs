use std::libc::{uintptr_t, c_int, c_char};
use std::c_str::CString;
use std::path::BytesContainer;
use std::str;
use std::ptr;

#[link(name = "curl")]
extern {
    fn curl_easy_escape(h: uintptr_t, url: *c_char, length: c_int) -> *c_char;
    fn curl_easy_init() -> uintptr_t;
    fn curl_easy_cleanup(h: uintptr_t);
    fn curl_easy_duphandle(h: uintptr_t) -> uintptr_t;
    fn curl_easy_perform(h: uintptr_t) -> c_int;
    fn curl_easy_reset(h: uintptr_t);
    fn curl_easy_strerror(code: c_int) -> *c_char;
    fn curl_easy_setopt(h: uintptr_t, option: c_int, parameter: uintptr_t) -> c_int;
    fn curl_easy_unescape(h: uintptr_t, url: *c_char, inlength: c_int, outlength: *c_int) -> *c_char;
    fn curl_free(ptr: *c_char);
}

pub trait ToCurlOptParam {
    fn to_curl_opt_param(&self) -> uintptr_t;
}

impl ToCurlOptParam for int {
    fn to_curl_opt_param(&self) -> uintptr_t {
        *self as uintptr_t
    }
}

impl ToCurlOptParam for bool {
    fn to_curl_opt_param(&self) -> uintptr_t {
        match *self {
            true  => 1,
            false => 0
        }
    }
}

impl<'a> ToCurlOptParam for &'a str {
    fn to_curl_opt_param(&self) -> uintptr_t {
        let c_string = self.to_c_str();
        ptr::to_unsafe_ptr(&c_string.container_into_owned_bytes()[0]) as uintptr_t
    }
}

// NOTE: return [u8] as a *c_char will not guarantee a \0 byte at end.
//       So here I convert it to a CString.
impl<'a> ToCurlOptParam for &'a [u8] {
    fn to_curl_opt_param(&self) -> uintptr_t {
        let c_string = self.to_c_str();
        ptr::to_unsafe_ptr(&c_string.container_into_owned_bytes()[0]) as uintptr_t
    }
}

pub struct Curl {
    handle: uintptr_t,
}

impl Drop for Curl {
    fn drop(&mut self) {
        unsafe { curl_easy_cleanup(self.handle) }
    }
}

// TODO: add deriving
impl Curl {
    pub fn is_null(&self) -> bool {
        self.handle == 0
    }

    // FIXME: handle \x00 byte in string
    pub fn escape(&self, url: &str) -> ~str {
        let c_url = url.to_c_str();
        c_url.with_ref(|c_buf| {
                unsafe {
                    let ret = curl_easy_escape(self.handle, c_buf, url.len() as c_int);
                    let escaped_bytes = CString::new(ret, false).container_into_owned_bytes();
                    curl_free(ret);
                    str::from_utf8_owned(escaped_bytes)
                }
            })
    }

    pub fn init() -> Curl {
        let hd = unsafe { curl_easy_init() };
        Curl { handle: hd }
    }

    /// empty func; use Drop trait instead
    pub fn cleanup(&self) {
        // unsafe { curl_easy_cleanup(self.handle) }
    }

    pub fn duphandle(&self) -> Curl {
        let ret = unsafe { curl_easy_duphandle(self.handle) };
        Curl { handle: ret }
    }

    pub fn perform(&self) -> int {
        let ret = unsafe { curl_easy_perform(self.handle) };
        ret as int
    }

    pub fn setopt<T: ToCurlOptParam>(&self, option: c_int, param: T) -> int {
        unsafe {
            curl_easy_setopt(self.handle, option, param.to_curl_opt_param()) as int
        }
    }

    pub fn reset(&self) {
        unsafe { curl_easy_reset(self.handle) }
    }

    pub fn unescape(&self, url: &str) -> ~str {
        let c_url = url.to_c_str();
        let outlen: c_int = 0;  // does not need to be mut
        c_url.with_ref(|c_buf| {
                unsafe {
                    let ret = curl_easy_unescape(self.handle, c_buf, url.len() as c_int, &outlen);
                    let unescaped_url = str::raw::from_buf_len(ret as *u8, outlen as uint);
                    curl_free(ret);
                    unescaped_url
                }
            })
    }
}

pub fn init() -> Curl {
    let hd = unsafe { curl_easy_init() };
    Curl { handle: hd }
}

pub fn cleanup(_handle: Curl) {
    // let hd = handle.handle;
    // unsafe { curl_easy_cleanup(hd) }
}


pub fn perform(handle: Curl) {
    let hd = handle.handle;
    unsafe { curl_easy_perform(hd) };
}

pub fn strerror(code: int) -> ~str {
    unsafe {
        let cver = CString::new(curl_easy_strerror(code as c_int), false);
        str::from_utf8_owned(cver.container_into_owned_bytes())
    }
}

pub mod opt {
    use std::libc::types::os::arch::c95::c_int;

    pub static FILE : c_int = 10000 + 1;
    pub static URL : c_int = 10000 + 2;
    pub static PORT : c_int = 0 + 3;
    pub static PROXY : c_int = 10000 + 4;
    pub static USERPWD : c_int = 10000 + 5;
    pub static PROXYUSERPWD : c_int = 10000 + 6;
    pub static RANGE : c_int = 10000 + 7;
    pub static INFILE : c_int = 10000 + 9;
    pub static ERRORBUFFER : c_int = 10000 + 10;
    pub static WRITEFUNCTION : c_int = 20000 + 11;
    pub static READFUNCTION : c_int = 20000 + 12;
    pub static TIMEOUT : c_int = 0 + 13;
    pub static INFILESIZE : c_int = 0 + 14;
    pub static POSTFIELDS : c_int = 10000 + 15;
    pub static REFERER : c_int = 10000 + 16;
    pub static FTPPORT : c_int = 10000 + 17;
    pub static USERAGENT : c_int = 10000 + 18;
    pub static LOW_SPEED_LIMIT : c_int = 0 + 19;
    pub static LOW_SPEED_TIME : c_int = 0 + 20;
    pub static RESUME_FROM : c_int = 0 + 21;
    pub static COOKIE : c_int = 10000 + 22;
    pub static HTTPHEADER : c_int = 10000 + 23;
    pub static HTTPPOST : c_int = 10000 + 24;
    pub static SSLCERT : c_int = 10000 + 25;
    pub static KEYPASSWD : c_int = 10000 + 26;
    pub static CRLF : c_int = 0 + 27;
    pub static QUOTE : c_int = 10000 + 28;
    pub static WRITEHEADER : c_int = 10000 + 29;
    pub static COOKIEFILE : c_int = 10000 + 31;
    pub static SSLVERSION : c_int = 0 + 32;
    pub static TIMECONDITION : c_int = 0 + 33;
    pub static TIMEVALUE : c_int = 0 + 34;
    pub static CUSTOMREQUEST : c_int = 10000 + 36;
    pub static STDERR : c_int = 10000 + 37;
    pub static POSTQUOTE : c_int = 10000 + 39;
    pub static WRITEINFO : c_int = 10000 + 40;
    pub static VERBOSE : c_int = 0 + 41;
    pub static HEADER : c_int = 0 + 42;
    pub static NOPROGRESS : c_int = 0 + 43;
    pub static NOBODY : c_int = 0 + 44;
    pub static FAILONERROR : c_int = 0 + 45;
    pub static UPLOAD : c_int = 0 + 46;
    pub static POST : c_int = 0 + 47;
    pub static DIRLISTONLY : c_int = 0 + 48;
    pub static APPEND : c_int = 0 + 50;
    pub static NETRC : c_int = 0 + 51;
    pub static FOLLOWLOCATION : c_int = 0 + 52;
    pub static TRANSFERTEXT : c_int = 0 + 53;
    pub static PUT : c_int = 0 + 54;
    pub static PROGRESSFUNCTION : c_int = 20000 + 56;
    pub static PROGRESSDATA : c_int = 10000 + 57;
    pub static AUTOREFERER : c_int = 0 + 58;
    pub static PROXYPORT : c_int = 0 + 59;
    pub static POSTFIELDSIZE : c_int = 0 + 60;
    pub static HTTPPROXYTUNNEL : c_int = 0 + 61;
    pub static INTERFACE : c_int = 10000 + 62;
    pub static KRBLEVEL : c_int = 10000 + 63;
    pub static SSL_VERIFYPEER : c_int = 0 + 64;
    pub static CAINFO : c_int = 10000 + 65;
    pub static MAXREDIRS : c_int = 0 + 68;
    pub static FILETIME : c_int = 0 + 69;
    pub static TELNETOPTIONS : c_int = 10000 + 70;
    pub static MAXCONNECTS : c_int = 0 + 71;
    pub static CLOSEPOLICY : c_int = 0 + 72;
    pub static FRESH_CONNECT : c_int = 0 + 74;
    pub static FORBID_REUSE : c_int = 0 + 75;
    pub static RANDOM_FILE : c_int = 10000 + 76;
    pub static EGDSOCKET : c_int = 10000 + 77;
    pub static CONNECTTIMEOUT : c_int = 0 + 78;
    pub static HEADERFUNCTION : c_int = 20000 + 79;
    pub static HTTPGET : c_int = 0 + 80;
    pub static SSL_VERIFYHOST : c_int = 0 + 81;
    pub static COOKIEJAR : c_int = 10000 + 82;
    pub static SSL_CIPHER_LIST : c_int = 10000 + 83;
    pub static HTTP_VERSION : c_int = 0 + 84;
    pub static FTP_USE_EPSV : c_int = 0 + 85;
    pub static SSLCERTTYPE : c_int = 10000 + 86;
    pub static SSLKEY : c_int = 10000 + 87;
    pub static SSLKEYTYPE : c_int = 10000 + 88;
    pub static SSLENGINE : c_int = 10000 + 89;
    pub static SSLENGINE_DEFAULT : c_int = 0 + 90;
    pub static DNS_USE_GLOBAL_CACHE : c_int = 0 + 91;
    pub static DNS_CACHE_TIMEOUT : c_int = 0 + 92;
    pub static PREQUOTE : c_int = 10000 + 93;
    pub static DEBUGFUNCTION : c_int = 20000 + 94;
    pub static DEBUGDATA : c_int = 10000 + 95;
    pub static COOKIESESSION : c_int = 0 + 96;
    pub static CAPATH : c_int = 10000 + 97;
    pub static BUFFERSIZE : c_int = 0 + 98;
    pub static NOSIGNAL : c_int = 0 + 99;
    pub static SHARE : c_int = 10000 + 100;
    pub static PROXYTYPE : c_int = 0 + 101;
    pub static ACCEPT_ENCODING : c_int = 10000 + 102;
    pub static PRIVATE : c_int = 10000 + 103;
    pub static HTTP200ALIASES : c_int = 10000 + 104;
    pub static UNRESTRICTED_AUTH : c_int = 0 + 105;
    pub static FTP_USE_EPRT : c_int = 0 + 106;
    pub static HTTPAUTH : c_int = 0 + 107;
    pub static SSL_CTX_FUNCTION : c_int = 20000 + 108;
    pub static SSL_CTX_DATA : c_int = 10000 + 109;
    pub static FTP_CREATE_MISSING_DIRS : c_int = 0 + 110;
    pub static PROXYAUTH : c_int = 0 + 111;
    pub static FTP_RESPONSE_TIMEOUT : c_int = 0 + 112;
    pub static SERVER_RESPONSE_TIMEOUT : c_int = 0 + 112;
    pub static IPRESOLVE : c_int = 0 + 113;
    pub static MAXFILESIZE : c_int = 0 + 114;
    pub static INFILESIZE_LARGE : c_int = 30000 + 115;
    pub static RESUME_FROM_LARGE : c_int = 30000 + 116;
    pub static MAXFILESIZE_LARGE : c_int = 30000 + 117;
    pub static NETRC_FILE : c_int = 10000 + 118;
    pub static USE_SSL : c_int = 0 + 119;
    pub static POSTFIELDSIZE_LARGE : c_int = 30000 + 120;
    pub static TCP_NODELAY : c_int = 0 + 121;
    pub static FTPSSLAUTH : c_int = 0 + 129;
    pub static IOCTLFUNCTION : c_int = 20000 + 130;
    pub static IOCTLDATA : c_int = 10000 + 131;
    pub static FTP_ACCOUNT : c_int = 10000 + 134;
    pub static COOKIELIST : c_int = 10000 + 135;
    pub static IGNORE_CONTENT_LENGTH : c_int = 0 + 136;
    pub static FTP_SKIP_PASV_IP : c_int = 0 + 137;
    pub static FTP_FILEMETHOD : c_int = 0 + 138;
    pub static LOCALPORT : c_int = 0 + 139;
    pub static LOCALPORTRANGE : c_int = 0 + 140;
    pub static CONNECT_ONLY : c_int = 0 + 141;
    pub static CONV_FROM_NETWORK_FUNCTION : c_int = 20000 + 142;
    pub static CONV_TO_NETWORK_FUNCTION : c_int = 20000 + 143;
    pub static CONV_FROM_UTF8_FUNCTION : c_int = 20000 + 144;
    pub static MAX_SEND_SPEED_LARGE : c_int = 30000 + 145;
    pub static MAX_RECV_SPEED_LARGE : c_int = 30000 + 146;
    pub static FTP_ALTERNATIVE_TO_USER : c_int = 10000 + 147;
    pub static SOCKOPTFUNCTION : c_int = 20000 + 148;
    pub static SOCKOPTDATA : c_int = 10000 + 149;
    pub static SSL_SESSIONID_CACHE : c_int = 0 + 150;
    pub static SSH_AUTH_TYPES : c_int = 0 + 151;
    pub static SSH_PUBLIC_KEYFILE : c_int = 10000 + 152;
    pub static SSH_PRIVATE_KEYFILE : c_int = 10000 + 153;
    pub static FTP_SSL_CCC : c_int = 0 + 154;
    pub static TIMEOUT_MS : c_int = 0 + 155;
    pub static CONNECTTIMEOUT_MS : c_int = 0 + 156;
    pub static HTTP_TRANSFER_DECODING : c_int = 0 + 157;
    pub static HTTP_CONTENT_DECODING : c_int = 0 + 158;
    pub static NEW_FILE_PERMS : c_int = 0 + 159;
    pub static NEW_DIRECTORY_PERMS : c_int = 0 + 160;
    pub static POSTREDIR : c_int = 0 + 161;
    pub static SSH_HOST_PUBLIC_KEY_MD5 : c_int = 10000 + 162;
    pub static OPENSOCKETFUNCTION : c_int = 20000 + 163;
    pub static OPENSOCKETDATA : c_int = 10000 + 164;
    pub static COPYPOSTFIELDS : c_int = 10000 + 165;
    pub static PROXY_TRANSFER_MODE : c_int = 0 + 166;
    pub static SEEKFUNCTION : c_int = 20000 + 167;
    pub static SEEKDATA : c_int = 10000 + 168;
    pub static CRLFILE : c_int = 10000 + 169;
    pub static ISSUERCERT : c_int = 10000 + 170;
    pub static ADDRESS_SCOPE : c_int = 0 + 171;
    pub static CERTINFO : c_int = 0 + 172;
    pub static USERNAME : c_int = 10000 + 173;
    pub static PASSWORD : c_int = 10000 + 174;
    pub static PROXYUSERNAME : c_int = 10000 + 175;
    pub static PROXYPASSWORD : c_int = 10000 + 176;
    pub static NOPROXY : c_int = 10000 + 177;
    pub static TFTP_BLKSIZE : c_int = 0 + 178;
    pub static SOCKS5_GSSAPI_SERVICE : c_int = 10000 + 179;
    pub static SOCKS5_GSSAPI_NEC : c_int = 0 + 180;
    pub static PROTOCOLS : c_int = 0 + 181;
    pub static REDIR_PROTOCOLS : c_int = 0 + 182;
    pub static SSH_KNOWNHOSTS : c_int = 10000 + 183;
    pub static SSH_KEYFUNCTION : c_int = 20000 + 184;
    pub static SSH_KEYDATA : c_int = 10000 + 185;
    pub static MAIL_FROM : c_int = 10000 + 186;
    pub static MAIL_RCPT : c_int = 10000 + 187;
    pub static FTP_USE_PRET : c_int = 0 + 188;
    pub static RTSP_REQUEST : c_int = 0 + 189;
    pub static RTSP_SESSION_ID : c_int = 10000 + 190;
    pub static RTSP_STREAM_URI : c_int = 10000 + 191;
    pub static RTSP_TRANSPORT : c_int = 10000 + 192;
    pub static RTSP_CLIENT_CSEQ : c_int = 0 + 193;
    pub static RTSP_SERVER_CSEQ : c_int = 0 + 194;
    pub static INTERLEAVEDATA : c_int = 10000 + 195;
    pub static INTERLEAVEFUNCTION : c_int = 20000 + 196;
    pub static WILDCARDMATCH : c_int = 0 + 197;
    pub static CHUNK_BGN_FUNCTION : c_int = 20000 + 198;
    pub static CHUNK_END_FUNCTION : c_int = 20000 + 199;
    pub static FNMATCH_FUNCTION : c_int = 20000 + 200;
    pub static CHUNK_DATA : c_int = 10000 + 201;
    pub static FNMATCH_DATA : c_int = 10000 + 202;
    pub static RESOLVE : c_int = 10000 + 203;
    pub static TLSAUTH_USERNAME : c_int = 10000 + 204;
    pub static TLSAUTH_PASSWORD : c_int = 10000 + 205;
    pub static TLSAUTH_TYPE : c_int = 10000 + 206;
    pub static TRANSFER_ENCODING : c_int = 0 + 207;
    pub static CLOSESOCKETFUNCTION : c_int = 20000 + 208;
    pub static CLOSESOCKETDATA : c_int = 10000 + 209;
    pub static GSSAPI_DELEGATION : c_int = 0 + 210;
    pub static DNS_SERVERS : c_int = 10000 + 211;
    pub static ACCEPTTIMEOUT_MS : c_int = 0 + 212;
    pub static TCP_KEEPALIVE : c_int = 0 + 213;
    pub static TCP_KEEPIDLE : c_int = 0 + 214;
    pub static TCP_KEEPINTVL : c_int = 0 + 215;
    pub static SSL_OPTIONS : c_int = 0 + 216;
    pub static MAIL_AUTH : c_int = 10000 + 217;
}
