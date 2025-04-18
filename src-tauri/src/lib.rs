mod keygrabber;

use keygrabber::event::Event;
use tauri::ipc::Channel;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn subscribe_to_key_events(channel: Channel<Event>) -> Result<(), ()> {
    let mut rx = keygrabber::get_channel();
    tauri::async_runtime::spawn(async move {
        loop {
            let event = rx.recv().await.unwrap();
            channel.send(event).unwrap();
        }
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, subscribe_to_key_events])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
