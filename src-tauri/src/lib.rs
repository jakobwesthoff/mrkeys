use std::sync::Mutex;

mod commands;
mod keygrabber;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(commands::KeygrabberSubscriptions::default()))
        .invoke_handler(tauri::generate_handler![
            commands::register_keygrabber,
            commands::unregister_keygrabber
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
