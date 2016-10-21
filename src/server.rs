use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri::AbsolutePath;
use hyper;

use twitter::send_tweet;
use prensa_corrupta::new_tweet;
use oauth::Credentials;

pub fn listen(port: u16, authorized_path: String, credentials: Credentials) {
    let bind_address = ("0.0.0.0", port);
    println!("Server listening on: {:?}", bind_address);
    Server::http(bind_address)
        .unwrap()
        .handle(move |req: Request, mut res: Response| {
            match (req.uri, req.method) {
                (AbsolutePath(ref path), hyper::Post) if path == authorized_path.as_str() => {
                    match send_tweet(&credentials, &new_tweet(&credentials)) {
                        Ok(_) => *res.status_mut() = StatusCode::Created,
                        Err(_) => *res.status_mut() = StatusCode::InternalServerError,
                    }
                }
                _ => *res.status_mut() = StatusCode::NotFound,
            };
        })
        .unwrap();
}
