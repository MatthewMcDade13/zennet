use std::{
    io::Cursor,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};

use anyhow::bail;
use bytes::{Bytes, BytesMut};
use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    name_server::TokioRuntimeProvider,
    Resolver, TokioAsyncResolver,
};
use http::Uri;
use tokio::runtime::Runtime;

use crate::packet::PacketBuf;

#[derive(Debug, Copy, Clone)]
pub enum Protocol {
    Tcp,
    Http,
    Https,
    // my own protocl
    Zen,
}

pub async fn client(path: &str, port: u16) -> anyhow::Result<NetSocket> {
    let addr = format!("{}:{}", path, port);
    let stream = tokio::net::TcpStream::connect(addr).await?;
    let stream = NetSocket::from(stream);

    Ok(stream)
}

pub async fn server(path: &str, port: u32) -> anyhow::Result<Server> {
    let addr = format!("{}:{}", path, port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let server = Server::from(listener);
    Ok(server)
}

// pub async fn request(stream: &mut tokio::net::TcpStream) -> anyhow::Result<()> {
//     // loop {
//     let req = read_request(stream).await?;
//     if req.len == 0 {
//         return Ok(());
//     }
//     println!("From Client: {}", req.body);
//
//     write_request_str(stream, "Ok(201)").await?;
//     Ok(())
// }
//
// async fn write_request_str(stream: &mut tokio::net::TcpStream, msg: &str) -> anyhow::Result<()> {
//     let len = msg.len() as u32;
//     let cap = (len + Request::HEADER_SIZE) as usize;
//     let mut wbuf: Vec<u8> = Vec::with_capacity(cap);
//     {
//         let len_bytes = len.to_ne_bytes();
//         wbuf.extend_from_slice(len_bytes.as_slice());
//     }
//
//     wbuf.extend_from_slice(msg.as_bytes());
//
//     let writesize = len as usize + 4;
//     let mut i = 0;
//
//     while i < writesize {
//         stream.writable().await?;
//
//         match stream.try_write(&mut wbuf[i..]) {
//             std::io::Result::Ok(n) => {
//                 i = i + n;
//             }
//             Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
//             Err(e) => {
//                 bail!("{}", e);
//             }
//         }
//     }
//     Ok(())
// }
//
// pub async fn read_request(stream: &mut tokio::net::TcpStream) -> anyhow::Result<Request> {
//     let mut buf = [0u8; 4096];
//     let mut i = 0;
//
//     let mut buflen: Option<u32> = None;
//
//     while i < buf.len() {
//         stream.readable().await?;
//         match stream.try_read(&mut buf[i..]) {
//             std::io::Result::Ok(n) => {
//                 i = i + n;
//             }
//             Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
//             Err(e) => {
//                 bail!("{}", e);
//             }
//         }
//
//         if i >= 4 && buflen.is_none() {
//             let h = &buf[0..4];
//             let mut header = [0u8; 4];
//             header.copy_from_slice(h);
//
//             let len = u32::from_ne_bytes(header);
//             buflen = Some(len);
//         }
//
//         if let Some(len) = buflen {
//             let len = len as usize;
//             if i >= len + 4 {
//                 let body = buf[4..i].to_vec();
//                 let body = String::from_utf8(body)?;
//                 return Ok(Request {
//                     len: len as u32,
//                     body,
//                 });
//             }
//         }
//     }
//     Ok(Request {
//         len: 0,
//         body: String::new(),
//     })
// }

// pub fn read_request_std(stream: &mut TcpStream) -> anyhow::Result<Request> {
//     let len = read_all(stream, Request::HEADER_SIZE)?;
//     let len = len.as_bytes();
//     let mut len_bytes = [0u8; 4];
//     len_bytes.copy_from_slice(len);
//     let len = u32::from_ne_bytes(len_bytes);
//
//     println!("HEADER BYTES: {}", len);
//     let body = read_all(stream, len)?;
//     Ok(Request { len, body })
// }
//
// pub fn write_request_std(stream: &mut TcpStream, msg: &str) -> anyhow::Result<()> {
//     let len = msg.len() as u32;
//     let cap = (len + Request::HEADER_SIZE) as usize;
//     let mut wbuf: Vec<u8> = Vec::with_capacity(cap);
//     {
//         let len_bytes = len.to_ne_bytes();
//         wbuf.extend_from_slice(len_bytes.as_slice());
//     }
//
//     wbuf.extend_from_slice(msg.as_bytes());
//     stream.write_all(wbuf.as_slice())?;
//
//     Ok(())
// }

#[derive(Debug, Clone)]
pub struct NetSocket(pub Arc<tokio::net::TcpStream>);

impl NetSocket {
    pub async fn write_all(&self, buf: &[u8]) -> anyhow::Result<()> {
        let len = buf.len();
        let mut wbuf: Vec<u8> = Vec::with_capacity(len);

        wbuf.extend_from_slice(buf);

        let mut i = 0;
        let stream = &self.0;

        while i < len {
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

    #[inline]
    pub async fn write_str(&self, msg: &str) -> anyhow::Result<()> {
        let bytes = msg.as_bytes();
        self.write_all(bytes).await
    }

    pub async fn readall(&self) -> anyhow::Result<Vec<u8>> {
        // let mut buf = BytesMut::with_capacity(4096);
        let mut buf = vec![0u8; 4096];
        let mut i = 0;

        let stream = &self.0;

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
        }
        println!("{:?}", buf);
        Ok(buf)
        // Ok(PacketBuf::from(buf))
    }
}
unsafe impl Send for NetSocket {}
unsafe impl Sync for NetSocket {}

impl From<tokio::net::TcpStream> for NetSocket {
    fn from(value: tokio::net::TcpStream) -> Self {
        Self(Arc::new(value))
    }
}

pub struct Server {
    pub listener: tokio::net::TcpListener,
    pub conns: Vec<NetSocket>,
    dns_resolver: Resolver,
}

impl Server {
    pub async fn new() -> anyhow::Result<Self> {
        Self::from_addr("0.0.0.0:1234").await
    }

    pub async fn from_addr(addr: &str) -> anyhow::Result<Self> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let s = Self {
            listener,
            conns: Vec::with_capacity(8),
            dns_resolver: Resolver::default()?,
        };
        Ok(s)
    }
}

unsafe impl Send for Server {}
unsafe impl Sync for Server {}

impl From<tokio::net::TcpListener> for Server {
    fn from(listener: tokio::net::TcpListener) -> Self {
        Self {
            listener,
            conns: Vec::with_capacity(8),
            dns_resolver: Resolver::default().unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct DnsResolver(Arc<TokioAsyncResolver>);

impl DnsResolver {
    pub fn default() -> anyhow::Result<Self> {
        let s = Self(Arc::new(TokioAsyncResolver::tokio(
            ResolverConfig::default(),
            ResolverOpts::default(),
        )));
        Ok(s)
    }

    pub async fn lookup_host(&self, uri: &str) -> anyhow::Result<Vec<SocketAddr>> {
        let mut soks = Vec::new();
        for addr in tokio::net::lookup_host(uri).await? {
            // if let SocketAddr::V4(v4addr) = addr {
            println!("{}", addr);
            soks.push(addr);
            // }
        }
        Ok(soks)
    }

    // pub async fn lookup_ip(&self, uri: &Uri) -> anyhow::Result<Vec<SocketAddr>> {
    //     let response = if let Some(host) = uri.host() {
    //         let host = format!("{}.", host);
    //         self.0.lookup_ip(host).await?
    //     } else {
    //         bail!("Unable to lookup ip of: {}", uri.to_string())
    //     };
    //
    //     let mut addrs = Vec::new();
    //     for addr in response.iter() {
    //         // println!("{}", addr.to_string());
    //         if let IpAddr::V4(v4addr) = addr {
    //             let s = &v4addr.to_string();
    //             let s: SocketAddr = addrs.push(s)
    //             // return Ok(v4addr);
    //         }
    //     }
    //
    //     Ok(addrs)
    // bail!(
    //     "Unable to get any ipv4 address from dns lookup for uri: {}",
    //     uri.to_string()
    // )
    // }
}

// pub fn resolve_domain(uri: &Uri) -> anyhow::Result<String> {
//     let resolver = Resolver::default();
// }
