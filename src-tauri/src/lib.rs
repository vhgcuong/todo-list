// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rusqlite::{Result, Connection};
mod commands;
mod models;
mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(db::Db(std::sync::Mutex::new(
            Connection::open("app_database.db").unwrap(),
        )))
        .invoke_handler(tauri::generate_handler![commands::todos, commands::change_done])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
