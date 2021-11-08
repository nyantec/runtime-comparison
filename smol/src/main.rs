use smol::{io, net};
use smol::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::block_on(async {
        let listener = net::TcpListener::bind("127.0.0.1:8153").await?;
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            let (mut reader, mut writer) = smol::io::split(stream);
            io::copy(&mut reader, &mut writer).await?;
        }

        Ok(())
    })
}
