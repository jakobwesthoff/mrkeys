use std::{sync::Arc, thread};

use lazy_static::lazy_static;
use rdev::{Event, EventType, listen};
use tokio::sync::{Mutex, broadcast};

lazy_static! {
    static ref SENDER: Mutex<Option<Arc<broadcast::Sender<Event>>>> = Mutex::new(None);
}

pub async fn subscribe() -> broadcast::Receiver<Event> {
    let mut possible_sender = SENDER.lock().await;
    if possible_sender.is_none() {
        let (tx, _) = broadcast::channel(32);
        let sender = Arc::new(tx);
        possible_sender.replace(sender.clone());

        thread::spawn(move || {
            if let Err(error) = listen(move |event| match event.event_type {
                EventType::KeyPress(_) | EventType::KeyRelease(_) => {
                    sender.send(event).unwrap();
                }
                _ => {}
            }) {
                println!("Error: {:?}", error)
            }
        });
    }

    possible_sender.as_ref().unwrap().clone().subscribe()
}
