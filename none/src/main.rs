fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = std::net::TcpListener::bind("127.0.0.1:8156")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream = stream?;
        std::io::copy(&mut &stream, &mut &stream)?;
    }
    Ok(())
}
