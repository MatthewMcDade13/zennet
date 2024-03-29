use http::Uri;
use tokio::runtime::Runtime;
use zennet::{
    conn::{DnsResolver, NetSocket},
    http1::client::Client,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let b = Client::oneshot_get("www.duckduckgo.com:80").await?;
    println!("{:?}", b);

    // let dns = DnsResolver::default()?;
    // let uri = "www.google.com".parse::<Uri>()?;
    // let addr = dns.lookup_ip(&uri).await?;
    // let saddr = format!("{}:443", addr.to_string());
    // let s = tokio::net::TcpStream::connect(saddr).await?;
    // let c = NetSocket::from(s);
    println!("{}", String::from_utf8(b)?);
    Ok(())
}
