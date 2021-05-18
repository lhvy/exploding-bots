use std::io::BufReader;
use std::net::TcpStream;
use types::{Event, InitialState};

const NAME: &str = "human client";

fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:2021")?;

    jsonl::write(&mut stream, &NAME)?;

    let mut stream = BufReader::new(&mut stream);

    let initial_state: InitialState = jsonl::read(&mut stream)?;
    dbg!(initial_state);

    loop {
        let event: Event = jsonl::read(&mut stream)?;
        dbg!(event);
    }
}
