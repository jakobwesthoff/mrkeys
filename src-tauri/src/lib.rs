mod keygrabber;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|_app| {
            tauri::async_runtime::spawn(async move {
                let mut rx = keygrabber::subscribe().await;
                tokio::spawn(async move {
                    loop {
                        let event = rx.recv().await.unwrap();
                        println!("{:?}", event);
                    }
                })
                .await
                .unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
