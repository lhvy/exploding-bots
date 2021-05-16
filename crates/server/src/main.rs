use std::{io::BufReader, net::TcpListener};
use types::User;
use uuid::Uuid;

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2021")?;
    let mut users = Vec::new();
    for stream in listener.incoming() {
        let stream = stream?;
        users.push(User {
            id: Uuid::new_v4(),
            name: jsonl::read(BufReader::new(stream))?,
        });
    }
    Ok(())
}
