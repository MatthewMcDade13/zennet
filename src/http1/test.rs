#[cfg(test)]
mod tests {
    use http::Uri;

    use crate::conn::{DnsResolver, NetSocket};

    use super::*;

    // #[tokio::test]
    // async fn client_get() -> anyhow::Result<()> {
    //     let dns = DnsResolver::default()?;
    //     let uri = "www.google.com".parse::<Uri>()?;
    //     let addr = dns.lookup_ip(&uri)?;
    //     // let saddr = format!("{}:443", addr.to_string());
    //     let s = tokio::net::TcpStream::connect("www.duckduckgo.com:443").await?;
    //     let c = NetSocket::from(s);
    //     let b = c.readall().await?;
    //     println!("{}", String::from_utf8(b.0.to_vec())?);
    //     Ok(())
    // }
}
