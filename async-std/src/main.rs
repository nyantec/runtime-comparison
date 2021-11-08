use async_std::{io, net, task};
use async_std::stream::StreamExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    task::block_on(async {
        let listener = net::TcpListener::bind("127.0.0.1:8154").await?;
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            io::copy(&mut &stream, &mut &stream).await?;
        }

        Ok(())
    })
}
