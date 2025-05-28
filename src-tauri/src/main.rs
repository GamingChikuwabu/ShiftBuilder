mod commands;
mod auth;
mod sheet;
mod drive;
mod server;
mod starter;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn main() {
    tauri::Builder::default()
        .setup(|app| {
            println!("アプリケーションを初期化しています...");
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(starter::init_app(app.handle()))?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::start_google_login,
            commands::is_logged_in,
            commands::logout, 
            commands::get_current_user,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}