use flume::Receiver;

pub(crate) fn run(ui_event_rx: Receiver<Event>) {
    for event in ui_event_rx {
        match event {
            Event::Joined(name) => println!("{} joined the game", name),
        }
    }
}

pub(crate) enum Event {
    Joined(String),
}
