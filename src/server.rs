use std::{future::Future, net::SocketAddr};

use tokio::net::TcpStream;

use crate::conn::{request, Conn, ConnStream, Request};

pub struct Server {
    connections: Vec<Conn>,
    listener: tokio::net::TcpListener,
}

impl Server {
    pub async fn new(address: &str) -> anyhow::Result<Self> {
        let listener = tokio::net::TcpListener::bind(address).await?;
        let s = Self {
            connections: Vec::new(),
            listener,
        };
        Ok(s)
    }

    pub async fn accept<F, Fut>(&mut self, mut connection_handler: F) -> anyhow::Result<()>
    where
        F: FnMut(&mut TcpStream) -> Fut,
        Fut: Future<Output = anyhow::Result<()>>,
    {
        while let Ok((mut stream, _)) = self.listener.accept().await {
            let _ = connection_handler(&mut stream).await;
        }
        Ok(())
    }

    pub async fn run<F>(&mut self) -> anyhow::Result<()> {
        while let Ok((mut stream, _)) = self.listener.accept().await {
            // let mut conn = Conn::from(stream);
            // self.connections.push(conn);
            tokio::spawn(async move {
                let _ = request(&mut stream).await;
            });
        }
        Ok(())
    }
}
