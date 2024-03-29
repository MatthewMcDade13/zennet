use std::{collections::HashMap, fmt::Display};

pub mod client;
pub mod header;
mod test;

#[derive(Debug, Copy, Clone)]
pub enum Method {
    Get,
    Put,
    Post,
    Delete,
    Connect,
    Head,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Method::Get => "GET",
            Method::Put => "PUT",
            Method::Post => "POST",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Head => "HEAD",
        };
        write!(f, "{}", s)
    }
}

// #[derive(Debug, Clone)]
// pub struct Request {
//     method: Method,
//     header: Header,
//     body: Vec<u8>,
// }
//
// impl Request {
//     pub fn get(uri: &http::Uri) -> Request {
//         let mut req = String::new();
//         req.push_str(&format!("GET {} HTTP/1.1\r\n", uri.path()));
//         req.push_str(&format!("Host: {}\r\n", uri.host().unwrap()));
//         req.push_str("\r\n");
//     }
// }
//
// #[derive(Debug, Default, Clone)]
// pub struct Header {
//     startline: String,
//     data: HashMap<String, String>,
// }
//
// impl Header {
//     #[inline]
//     pub fn new() -> Self {
//         Self::default()
//     }
// }
