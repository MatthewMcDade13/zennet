use http::{uri::Scheme, Request, Response, Uri};

use crate::conn::Conn;

/// String-based Http/1.x Message
#[derive(Default, Debug, Clone)]
pub struct HttpMessage {
    status: u32,
    version: String,
    headers: String,
    body: String,
}

pub async fn get(addr: &str) -> anyhow::Result<HttpMessage> {
    let uri = addr.parse::<Uri>()?;
    let is_https = if let Some("https") = uri.scheme_str() {
        true
    } else {
        false
    };
    let port = if let Some(p) = uri.port_u16() {
        p
    } else if is_https {
        443
    } else {
        80
    };

    let path = uri.to_string();
    println!("URI: {}", path);

    let conn = Conn::client(&path, port);

    let req = Request::builder();

    Ok(HttpMessage::default())
}
