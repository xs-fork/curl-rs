use libc::types::os::arch::c95::c_int;

static CURLINFO_STRING : c_int = 0x100000;
static CURLINFO_LONG   : c_int = 0x200000;
static CURLINFO_DOUBLE : c_int = 0x300000;
static CURLINFO_SLIST  : c_int = 0x400000;

// #define CURLINFO_MASK     0x0fffff
// #define CURLINFO_TYPEMASK 0xf00000

pub static EFFECTIVE_URL    : c_int = CURLINFO_STRING + 1;
pub static RESPONSE_CODE    : c_int = CURLINFO_LONG   + 2;
pub static TOTAL_TIME       : c_int = CURLINFO_DOUBLE + 3;
pub static NAMELOOKUP_TIME  : c_int = CURLINFO_DOUBLE + 4;
pub static CONNECT_TIME     : c_int = CURLINFO_DOUBLE + 5;
pub static PRETRANSFER_TIME : c_int = CURLINFO_DOUBLE + 6;
pub static SIZE_UPLOAD      : c_int = CURLINFO_DOUBLE + 7;
pub static SIZE_DOWNLOAD    : c_int = CURLINFO_DOUBLE + 8;
pub static SPEED_DOWNLOAD   : c_int = CURLINFO_DOUBLE + 9;
pub static SPEED_UPLOAD     : c_int = CURLINFO_DOUBLE + 10;
pub static HEADER_SIZE      : c_int = CURLINFO_LONG   + 11;
pub static REQUEST_SIZE     : c_int = CURLINFO_LONG   + 12;
pub static SSL_VERIFYRESULT : c_int = CURLINFO_LONG   + 13;
pub static FILETIME         : c_int = CURLINFO_LONG   + 14;
pub static CONTENT_LENGTH_DOWNLOAD   : c_int = CURLINFO_DOUBLE + 15;
pub static CONTENT_LENGTH_UPLOAD     : c_int = CURLINFO_DOUBLE + 16;
pub static STARTTRANSFER_TIME : c_int = CURLINFO_DOUBLE + 17;
pub static CONTENT_TYPE     : c_int = CURLINFO_STRING + 18;
pub static REDIRECT_TIME    : c_int = CURLINFO_DOUBLE + 19;
pub static REDIRECT_COUNT   : c_int = CURLINFO_LONG   + 20;
pub static PRIVATE          : c_int = CURLINFO_STRING + 21;
pub static HTTP_CONNECTCODE : c_int = CURLINFO_LONG   + 22;
pub static HTTPAUTH_AVAIL   : c_int = CURLINFO_LONG   + 23;
pub static PROXYAUTH_AVAIL  : c_int = CURLINFO_LONG   + 24;
pub static OS_ERRNO         : c_int = CURLINFO_LONG   + 25;
pub static NUM_CONNECTS     : c_int = CURLINFO_LONG   + 26;
pub static SSL_ENGINES      : c_int = CURLINFO_SLIST  + 27;
pub static COOKIELIST       : c_int = CURLINFO_SLIST  + 28;
pub static LASTSOCKET       : c_int = CURLINFO_LONG   + 29;
pub static FTP_ENTRY_PATH   : c_int = CURLINFO_STRING + 30;
pub static REDIRECT_URL     : c_int = CURLINFO_STRING + 31;
pub static PRIMARY_IP       : c_int = CURLINFO_STRING + 32;
pub static APPCONNECT_TIME  : c_int = CURLINFO_DOUBLE + 33;
pub static CERTINFO         : c_int = CURLINFO_SLIST  + 34;
pub static CONDITION_UNMET  : c_int = CURLINFO_LONG   + 35;
pub static RTSP_SESSION_ID  : c_int = CURLINFO_STRING + 36;
pub static RTSP_CLIENT_CSEQ : c_int = CURLINFO_LONG   + 37;
pub static RTSP_SERVER_CSEQ : c_int = CURLINFO_LONG   + 38;
pub static RTSP_CSEQ_RECV   : c_int = CURLINFO_LONG   + 39;
pub static PRIMARY_PORT     : c_int = CURLINFO_LONG   + 40;
pub static LOCAL_IP         : c_int = CURLINFO_STRING + 41;
pub static LOCAL_PORT       : c_int = CURLINFO_LONG   + 42;
  /* Fill in new entries below here! */

/* CURLINFO_RESPONSE_CODE is the new name for the option previously known as
 pub static HTTP_CODE */
pub static HTTP_CODE : c_int = RESPONSE_CODE;
