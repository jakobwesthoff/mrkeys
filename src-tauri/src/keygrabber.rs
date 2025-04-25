use std::collections::HashSet;
use std::{sync::Arc, thread};

use lazy_static::lazy_static;
use rdev::{listen, Event, EventType, Key};
use tokio::sync::{broadcast, Mutex};

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
            let mut pressed_keys: HashSet<Key> = HashSet::new();
            if let Err(error) = listen(move |event| match event.event_type {
                EventType::KeyPress(key) => {
                    if pressed_keys.insert(key) {
                        sender.send(event).unwrap();
                    }
                }
                EventType::KeyRelease(key) => {
                    pressed_keys.remove(&key);
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
