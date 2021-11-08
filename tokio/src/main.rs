fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();

    runtime.block_on(async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8155").await?;

        loop {
            let stream = listener.accept().await?.0;
            let (mut reader, mut writer) = tokio::io::split(stream);
            tokio::io::copy(&mut reader, &mut writer).await?;
        }
    })
}
