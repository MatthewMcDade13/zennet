use anyhow::bail;
use std::{
    cell::{Ref, RefCell, RefMut},
    io::Write,
    net::{SocketAddr, TcpStream},
    sync::Arc,
};

use crate::read_all;

#[derive(Debug, Copy, Clone)]
pub enum Protocol {
    Tcp,
    Http,
    Https,
    // my own protocl
    Zen,
}

#[derive(Debug, PartialEq)]
pub enum RequestState {
    Good,
    Disconnected,
}

#[derive(Debug, Clone)]
pub struct Request {
    // header
    pub len: u32,
    pub body: String,
}

impl Request {
    pub const HEADER_SIZE: u32 = 4;
}

#[derive(Debug)]
pub enum Conn {
    Stream(ConnStream),
    Listener(ZenListener),
}

impl Conn {
    pub async fn client(path: &str, port: u16) -> anyhow::Result<Self> {
        let addr = format!("{}:{}", path, port);
        let stream = tokio::net::TcpStream::connect(addr).await?;
        let stream = ConnStream::from(stream);

        Ok(Self::Stream(stream))
    }

    pub async fn server(path: &str, port: u32) -> anyhow::Result<Self> {
        let addr = format!("{}:{}", path, port);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let listener = ZenListener::from(listener);
        let s = Self::Listener(listener);
        Ok(s)
    }
}

impl From<tokio::net::TcpStream> for Conn {
    fn from(value: tokio::net::TcpStream) -> Self {
        let stream = ConnStream::from(value);
        Self::Stream(stream)
    }
}

pub async fn request(stream: &mut tokio::net::TcpStream) -> anyhow::Result<()> {
    // loop {
    let req = read_request(stream).await?;
    if req.len == 0 {
        return Ok(());
    }
    println!("From Client: {}", req.body);

    write_request_str(stream, "Ok(201)").await?;
    Ok(())
}

async fn write_request_str(stream: &mut tokio::net::TcpStream, msg: &str) -> anyhow::Result<()> {
    let len = msg.len() as u32;
    let cap = (len + Request::HEADER_SIZE) as usize;
    let mut wbuf: Vec<u8> = Vec::with_capacity(cap);
    {
        let len_bytes = len.to_ne_bytes();
        wbuf.extend_from_slice(len_bytes.as_slice());
    }

    wbuf.extend_from_slice(msg.as_bytes());

    let writesize = len as usize + 4;
    let mut i = 0;

    while i < writesize {
        stream.writable().await?;

        match stream.try_write(&mut wbuf[i..]) {
            std::io::Result::Ok(n) => {
                i = i + n;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(e) => {
                bail!("{}", e);
            }
        }
    }
    Ok(())
}

pub async fn read_request(stream: &mut tokio::net::TcpStream) -> anyhow::Result<Request> {
    let mut buf = [0u8; 4096];
    let mut i = 0;

    let mut buflen: Option<u32> = None;

    while i < buf.len() {
        stream.readable().await?;
        match stream.try_read(&mut buf[i..]) {
            std::io::Result::Ok(n) => {
                i = i + n;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(e) => {
                bail!("{}", e);
            }
        }

        if i >= 4 && buflen.is_none() {
            let h = &buf[0..4];
            let mut header = [0u8; 4];
            header.copy_from_slice(h);

            let len = u32::from_ne_bytes(header);
            buflen = Some(len);
        }

        if let Some(len) = buflen {
            let len = len as usize;
            if i >= len + 4 {
                let body = buf[4..i].to_vec();
                let body = String::from_utf8(body)?;
                return Ok(Request {
                    len: len as u32,
                    body,
                });
            }
        }
    }
    Ok(Request {
        len: 0,
        body: String::new(),
    })
}

pub fn read_request_std(stream: &mut TcpStream) -> anyhow::Result<Request> {
    let len = read_all(stream, Request::HEADER_SIZE)?;
    let len = len.as_bytes();
    let mut len_bytes = [0u8; 4];
    len_bytes.copy_from_slice(len);
    let len = u32::from_ne_bytes(len_bytes);

    println!("HEADER BYTES: {}", len);
    let body = read_all(stream, len)?;
    Ok(Request { len, body })
}

pub fn write_request_std(stream: &mut TcpStream, msg: &str) -> anyhow::Result<()> {
    let len = msg.len() as u32;
    let cap = (len + Request::HEADER_SIZE) as usize;
    let mut wbuf: Vec<u8> = Vec::with_capacity(cap);
    {
        let len_bytes = len.to_ne_bytes();
        wbuf.extend_from_slice(len_bytes.as_slice());
    }

    wbuf.extend_from_slice(msg.as_bytes());
    stream.write_all(wbuf.as_slice())?;

    Ok(())
}

#[derive(Debug)]
pub struct ConnStream(pub tokio::net::TcpStream);

impl ConnStream {
    pub async fn write_request_str(&mut self, msg: &str) -> anyhow::Result<()> {
        write_request_str(&mut self.0, msg).await
    }

    pub async fn read_request(&mut self) -> anyhow::Result<Request> {
        read_request(&mut self.0).await
    }
}

unsafe impl Send for ConnStream {}
unsafe impl Sync for ConnStream {}

impl From<tokio::net::TcpStream> for ConnStream {
    fn from(value: tokio::net::TcpStream) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct ZenListener(pub Arc<tokio::net::TcpListener>);

impl ZenListener {
    pub async fn new(addr: &str) -> anyhow::Result<Self> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let listener = Arc::new(listener);
        Ok(Self(listener))
    }
}

unsafe impl Send for ZenListener {}
unsafe impl Sync for ZenListener {}

impl From<tokio::net::TcpListener> for ZenListener {
    fn from(value: tokio::net::TcpListener) -> Self {
        Self(Arc::new(value))
    }
}
