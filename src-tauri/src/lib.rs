// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rusqlite::{Connection, Result, Error};


#[derive(Debug)]
struct Todo {
    is_done: bool,
    text: String
}

#[tauri::command]
fn get_todo() -> Vec<Todo> {
    vec![]
}

fn create_database() -> Result<()> {
    let conn = Connection::open("app_database.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            is_done BOOL DEFAULT FALSE
        )",
        [], // No parameters needed
    )?;

    println!("Database and table created successfully.");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Error> {
    create_database()?;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
