use flume::{Receiver, Selector, Sender};
use std::io::{self, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use types::User;
use uuid::Uuid;

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2021")?;

    let (start_game_tx, start_game_rx) = flume::bounded(1);
    let accept_connections_handle = thread::spawn(|| accept_connections(listener, start_game_rx));

    println!("Press enter to stop accepting new connections and start the game.");
    wait_for_enter()?;
    start_game_tx.send(()).unwrap();

    let clients = accept_connections_handle.join().unwrap()?;
    dbg!(&clients);

    Ok(())
}

fn accept_connections(
    listener: TcpListener,
    stop_accepting_rx: Receiver<()>,
) -> anyhow::Result<Vec<Client>> {
    let mut clients = Vec::new();
    let (stream_tx, stream_rx) = flume::bounded(1);
    thread::spawn(|| listen(listener, stream_tx));

    #[must_use]
    enum ControlFlow {
        Continue,
        Break,
    }

    loop {
        let control_flow = Selector::new()
            .recv(&stream_rx, |stream| -> anyhow::Result<_> {
                let mut stream = stream.unwrap();

                let name = jsonl::read(BufReader::new(&mut stream))?;
                let user = User {
                    id: Uuid::new_v4(),
                    name,
                };
                let client = Client { user, stream };

                clients.push(client);

                Ok(ControlFlow::Continue)
            })
            .recv(&stop_accepting_rx, |empty_tuple| {
                let () = empty_tuple.unwrap();
                Ok(ControlFlow::Break)
            })
            .wait()?;

        match control_flow {
            ControlFlow::Continue => {}
            ControlFlow::Break => break,
        }
    }

    Ok(clients)
}

fn listen(listener: TcpListener, stream_tx: Sender<TcpStream>) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => stream_tx.send(stream).unwrap(),
            Err(e) => eprintln!("Error: {:#?}", anyhow::Error::new(e)),
        }
    }
}

#[derive(Debug)]
struct Client {
    user: User,
    stream: TcpStream,
}

fn wait_for_enter() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}
