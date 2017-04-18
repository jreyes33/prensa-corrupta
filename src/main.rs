extern crate base64;
extern crate chrono;
extern crate crypto;
extern crate hyper;
extern crate hyper_native_tls;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

mod oauth;
mod prensa_corrupta;
mod server;
mod twitter;

use std::env;
use oauth::Credentials;

fn main() {
    let port = env::var("PORT").and_then(|p| Ok(p.parse::<u16>().unwrap())).unwrap_or(8080);
    let authorized_path = env::var("AUTHORIZED_PATH").unwrap();
    let credentials = Credentials {
        consumer_key: env::var("CONSUMER_KEY").unwrap(),
        consumer_secret: env::var("CONSUMER_SECRET").unwrap(),
        access_token: env::var("ACCESS_TOKEN").unwrap(),
        access_token_secret: env::var("ACCESS_TOKEN_SECRET").unwrap(),
    };
    prensa_corrupta::new_tweet(&credentials);
    server::listen(port, authorized_path, credentials);
}
