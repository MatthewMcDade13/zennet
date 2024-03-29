use std::sync::Arc;

use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};
use http::Uri;

use crate::{
    conn::{DnsResolver, NetSocket},
    packet::PacketBuf,
};

#[derive(Clone)]
pub struct Client {
    conn: NetSocket,
    dns_resolver: DnsResolver,
}

impl Client {
    pub async fn connect(addr: &str) -> anyhow::Result<Self> {
        let dns = DnsResolver::default()?;
        let remote_addr = dns.lookup_host(addr).await?;

        let socket = tokio::net::TcpStream::connect(remote_addr.as_slice()).await?;
        let conn = NetSocket::from(socket);
        let s = Self {
            conn,
            dns_resolver: dns,
        };
        Ok(s)
    }

    pub async fn oneshot_get(addr: &str) -> anyhow::Result<Vec<u8>> {
        let uri = addr.parse::<Uri>()?;

        let client = Self::connect(addr).await?;

        let path = {
            "/"
            // let p = uri.path_and_query().unwrap().to_string();
            // if p.len() == 0 {
            //     String::from("/")
            // } else {
            //     p
            // }
        };
        let mut req = String::new();
        let host = uri.host().unwrap();

        req.push_str(&format!("GET {} HTTP/1.1\r\n", path));
        req.push_str(&format!("Host: {}\r\n", uri.host().unwrap()));
        req.push_str("\r\n");
        println!("{}", req);

        client.conn.write_str(&req).await?;

        let response = client.conn.readall().await?;

        Ok(response)
    }

    // #[inline]
    // pub async fn write_str(&self, msg: &str) -> anyhow::Result<()> {
    //     self.conn.write_str(msg).await
    // }
    //
    // #[inline]
    // pub async fn write_all(&self, buf: &[u8]) -> anyhow::Result<()> {
    //     self.conn.write_all(buf).await
    // }
    //
    // #[inline]
    // pub async fn readall(&self) -> anyhow::Result<Vec<u8>> {
    //     self.conn.readall().await
    // }
}
