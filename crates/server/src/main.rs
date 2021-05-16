mod ui;

use flume::Sender;
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
    let (ui_event_tx, ui_event_rx) = flume::unbounded();

    thread::spawn(|| ui::run(ui_event_rx));

    for stream in listener.incoming() {
        let stream = stream?;
        let users = Arc::clone(&users);
        let ui_event_tx = ui_event_tx.clone();

        thread::spawn(|| {
            if let Err(e) = handle_connection(stream, users, ui_event_tx) {
                eprintln!("Error: {:#?}", e);
            }
        });
    }

    Ok(())
}

fn handle_connection(
    stream: TcpStream,
    users: Arc<Mutex<Vec<User>>>,
    ui_event_tx: Sender<ui::Event>,
) -> anyhow::Result<()> {
    let name: String = jsonl::read(BufReader::new(stream))?;

    users.lock().push(User {
        id: Uuid::new_v4(),
        name: name.clone(),
    });

    ui_event_tx.send(ui::Event::Joined(name)).unwrap();

    Ok(())
}
