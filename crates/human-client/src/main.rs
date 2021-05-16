use std::net::TcpStream;

const NAME: &str = "human client";

fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:2021")?;

    jsonl::write(&mut stream, &NAME)?;

    Ok(())
}
