#[macro_use]
extern crate log;

use argh::FromArgs;
use derive_more::From;
use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufStream;
use tokio::net::TcpListener;

mod links;
use links::link;

#[derive(FromArgs)]
/// Reach new heights.
struct Options {
    /// listen port
    #[argh(option, short = 'p')]
    port: u16,
}

#[derive(Debug, From)]
enum Error {
    Io(io::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    let options: Options = argh::from_env();

    let address = format!("0.0.0.0:{}", options.port);
    let mut listener = TcpListener::bind(address).await?;
    let mut links = links::Links::new();

    while let Ok((stream, _)) = listener.accept().await {
        let mut stream = BufStream::new(stream);
        let mut id = String::new();
        if let Err(e) = stream.read_line(&mut id).await {
            error!("Failed to read config. Error = {:?}", e);
            continue;
        }

        links.connections.insert(id.clone(), stream);
        let (inn, out) = match links.get_link(&id) {
            Ok(o) => o,
            Err(e) => {
                error!("Failed to link. Error = {:?}", e);
                continue;
            }
        };

        let transfer = link(inn, out);
        tokio::spawn(async {
            let o = transfer.await;
            info!("Done!!. Result = {:?}", o);
        });
    }

    Ok(())
}
