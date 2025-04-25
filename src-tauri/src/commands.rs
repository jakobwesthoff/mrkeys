use crate::keygrabber;
use rdev::Event;
use std::{collections::HashMap, sync::Mutex};
use tauri::{async_runtime::JoinHandle, ipc::Channel, State};
use uuid::Uuid;

#[derive(Default)]
pub struct KeygrabberSubscriptions {
    subscriptions: HashMap<String, JoinHandle<()>>,
}

#[tauri::command]
pub async fn register_keygrabber(
    state: State<'_, Mutex<KeygrabberSubscriptions>>,
    channel: Channel<String>,
) -> Result<String, ()> {
    let id = Uuid::new_v4();
    let mut rx = keygrabber::subscribe().await;
    let handle = tauri::async_runtime::spawn(async move {
        loop {
            let event = rx.recv().await.unwrap();
            println!("{:?}", event);
            channel.send(format!("{:?}", event)).unwrap();
        }
    });

    let mut state = state.lock().unwrap();
    state.subscriptions.insert(id.to_string(), handle);

    Ok(id.to_string())
}

#[tauri::command]
pub async fn unregister_keygrabber(
    state: State<'_, Mutex<KeygrabberSubscriptions>>,
    id: String,
) -> Result<bool, ()> {
    let mut state = state.lock().unwrap();
    if let Some((_k, v)) = state.subscriptions.remove_entry(&id) {
        v.abort();
        Ok(true)
    } else {
        Ok(false)
    }
}
