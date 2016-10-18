use base64;
use chrono::offset::utc::UTC;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use hyper::header::Authorization;
use hyper::method::Method;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fmt;
use url;

pub struct Credentials {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
}

pub fn auth(credentials: &Credentials, method: &Method, base_url: &str,
            params: &HashMap<&str, &str>) -> Authorization<String> {
    let nonce_string = thread_rng().gen_ascii_chars().take(32).collect::<String>();
    let oauth_header = OAuthHeader {
        credentials: credentials,
        nonce: nonce_string.as_str(),
        signature_method: "HMAC-SHA1",
        timestamp: UTC::now().timestamp(),
        version: "1.0",
        method: method,
        base_url: base_url,
        params: params
    };
    println!("{}", oauth_header);
    Authorization(oauth_header.to_string())
}

struct OAuthHeader<'a> {
    credentials: &'a Credentials,
    nonce: &'a str,
    signature_method: &'a str,
    timestamp: i64,
    version: &'a str,
    method: &'a Method,
    base_url: &'a str,
    params: &'a HashMap<&'a str, &'a str>,
}

impl<'a> OAuthHeader<'a> {
    fn sign(&self) -> String {
        let signature_base = format!("{}&{}&{}", self.method, url_encode(self.base_url),
                                     url_encode(self.params_string()));
        println!("{}", signature_base);
        let key = format!("{}&{}", self.credentials.consumer_secret,
                          self.credentials.access_token_secret);
        let mut hmac = Hmac::new(Sha1::new(), key.as_bytes());
        hmac.input(signature_base.as_bytes());
        let signature = hmac.result();
        url_encode(base64::encode(signature.code()))
    }

    fn params_string(&self) -> String {
        let timestamp_string = self.timestamp.to_string();
        let mut oauth_params = HashMap::new();
        oauth_params.insert("oauth_consumer_key", self.credentials.consumer_key.as_str());
        oauth_params.insert("oauth_nonce", self.nonce);
        oauth_params.insert("oauth_signature_method", "HMAC-SHA1");
        oauth_params.insert("oauth_timestamp", timestamp_string.as_str());
        oauth_params.insert("oauth_token", self.credentials.access_token.as_str());
        oauth_params.insert("oauth_version", "1.0");
        let mut params: Vec<String> = oauth_params.iter().chain(self.params.iter()).map({ |(&k, &v)|
            format!("{}={}", percent_encode(k), percent_encode(v))
        }).collect();
        params.sort();
        params.join("&")
    }
}

impl<'a> fmt::Display for OAuthHeader<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter,
               "OAuth oauth_consumer_key=\"{}\", oauth_nonce=\"{}\", oauth_signature=\"{}\", \
                oauth_signature_method=\"{}\", oauth_timestamp=\"{}\", oauth_token=\"{}\", \
                oauth_version=\"{}\"", self.credentials.consumer_key, self.nonce, self.sign(),
                self.signature_method, self.timestamp, self.credentials.access_token, self.version)
    }
}

fn url_encode<S>(string: S) -> String where S: Into<String> {
    url::form_urlencoded::byte_serialize(string.into().as_bytes()).collect::<String>()
}

fn percent_encode<S>(string: S) -> String where S: Into<String> {
    url::percent_encoding::percent_encode(string.into().as_bytes(), url::percent_encoding::QUERY_ENCODE_SET)
        .collect::<String>()
}
