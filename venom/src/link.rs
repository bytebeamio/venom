use std::io;
use smol::Async;
use std::net::TcpStream;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error = {0}")]
    Io(#[from] io::Error),
    #[error("Destination down")]
    DestinationDown,
}

#[derive(Debug)]
pub struct Link {
    rx_bandwidth: u64,
    tx_bandwidth: u64,
    halfopen: bool,
}

impl Link {
    pub fn new() -> Link {
        Link {
            rx_bandwidth: 0,
            tx_bandwidth: 0,
            halfopen: false
        }
    }

    pub async fn start(&mut self, mut downstream: Async<TcpStream>, mut upstream: Async<TcpStream>) -> Result<(), Error> {
        futures_util::io::copy(&mut downstream, &mut upstream).await?;
        Ok(())
    }
}

