use parking_lot::Mutex;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use types::User;
use uuid::Uuid;

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2021")?;
    let users = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        let users = Arc::clone(&users);

        thread::spawn(|| {
            if let Err(e) = handle_connection(stream, users) {
                eprintln!("Error: {:#?}", e);
            }
        });
    }

    Ok(())
}

fn handle_connection(stream: TcpStream, users: Arc<Mutex<Vec<User>>>) -> anyhow::Result<()> {
    let name = jsonl::read(BufReader::new(stream))?;

    users.lock().push(User {
        id: Uuid::new_v4(),
        name,
    });

    dbg!(&users);

    Ok(())
}
