use std::collections::HashMap;

use derive_more::From;
use tokio::io;
use tokio::io::BufStream;
use tokio::join;
use tokio::net::TcpStream;
use tokio::time::Duration;

#[derive(Debug, From)]
pub enum Error {
    Io(io::Error),
    NoLink,
    DestinationDown,
}

#[derive(Debug)]
pub struct Link {
    destination_id: String,
    rx_bandwidth: u64,
    tx_bandwidth: u64,
    halfopen: bool,
    halfopen_count: usize,
    halfopen_interval: Duration,
}

#[derive(Debug)]
pub struct Links {
    pub connections: HashMap<String, BufStream<TcpStream>>,
    pub links: HashMap<String, Link>,
}

impl Links {
    pub fn new() -> Links {
        Links {
            connections: HashMap::new(),
            links: HashMap::new(),
        }
    }

    pub fn get_link(
        &mut self,
        id: &str,
    ) -> Result<(BufStream<TcpStream>, BufStream<TcpStream>), Error> {
        let link = match self.links.get(id) {
            Some(l) => l,
            None => return Err(Error::NoLink),
        };

        let out = match self.connections.remove(&link.destination_id) {
            Some(o) => o,
            None => return Err(Error::DestinationDown),
        };

        let inn = self.connections.remove(id).unwrap();
        Ok((inn, out))
    }
}

pub async fn link(inn: BufStream<TcpStream>, out: BufStream<TcpStream>) -> Result<(), Error> {
    let mut inn = inn.into_inner();
    let (mut in_rx, mut in_tx) = inn.split();

    let mut out = out.into_inner();
    let (mut out_rx, mut out_tx) = out.split();

    let client_to_server = io::copy(&mut in_rx, &mut out_tx);
    let server_to_client = io::copy(&mut out_rx, &mut in_tx);

    let (a, b) = join!(client_to_server, server_to_client);
    a?;
    b?;

    Ok(())
}
