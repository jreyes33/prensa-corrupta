use hyper::client::Client;
use hyper::header::ContentType;
use hyper::method::Method;
use serde_json;
use std::collections::HashMap;
use std::io::Read;
use url::percent_encoding::{percent_encode, QUERY_ENCODE_SET};

use oauth::{auth, Credentials};

#[derive(Deserialize, Debug)]
struct Tweet {
    text: String
}

pub fn last_tweet(credentials: &Credentials, screen_name: &str) -> String {
    let client = Client::new();
    let method = Method::Get;
    let base_url = "https://api.twitter.com/1.1/statuses/user_timeline.json";
    let mut params = HashMap::new();
    params.insert("screen_name", screen_name);
    params.insert("exclude_replies", "true");
    params.insert("include_rts", "false");
    params.insert("count", "5");
    let url = format!("{}?{}", base_url, query_string(&params));
    let mut response = client.get(&url).header(auth(credentials, &method, base_url, &params))
        .send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).expect("Should not fail");
    let value: serde_json::value::Value = serde_json::from_str(&body).unwrap();
    let tweets = value.as_array().unwrap();
    let tweet: Tweet = serde_json::from_value(tweets[0].clone()).unwrap();
    tweet.text
}

pub fn send_tweet(credentials: &Credentials, text: &str) -> Result<&'static str, &'static str> {
    let client = Client::new();
    let method = Method::Post;
    let base_url = "https://api.twitter.com/1.1/statuses/update.json";
    let mut params = HashMap::new();
    params.insert("status", text);
    let payload = query_string(&params);
    let mut response = client.post(base_url)
        .header(auth(credentials, &method, base_url, &params))
        .header(ContentType::form_url_encoded())
        .body(&payload)
        .send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).expect("Should not fail");
    Ok("Success")
}

fn query_string(params: &HashMap<&str, &str>) -> String {
    params.iter().map({ |(&k, &v)|
        format!("{}={}",
                percent_encode(k.as_bytes(), QUERY_ENCODE_SET).collect::<String>(),
                percent_encode(v.as_bytes(), QUERY_ENCODE_SET).collect::<String>())
    }).collect::<Vec<String>>().join("&")
}
