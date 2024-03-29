use std::fmt::Display;

use http::Uri;

use crate::packet::PacketBuf;

use super::Method;

pub const VERSION_STR: &'static str = "HTTP/1.1";

#[derive(Debug, Clone)]
pub struct Header {
    method: Method,
    header: PacketBuf,
    body: PacketBuf,
}
