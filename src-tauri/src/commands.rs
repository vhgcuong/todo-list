use crate::{models::Todo, db::Db};
use tauri::State;
use rusqlite::{Connection, Result};

#[tauri::command]
pub fn todos(db: State<Db>) -> Result<Vec<Todo>, String> {
    let conn = db.0.lock().unwrap();

    let mut stmt = conn.prepare("SELECT id, text, is_done FROM todos")
        .map_err(|e| e.to_string())?;

    let todos_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get::<_, i32>(0)?,
            text: row.get::<_, String>(1)?,
            is_done: row.get::<_, i32>(2)? != 0,
        })
    })
        .map_err(|e| e.to_string())?;
    let todos: Vec<Todo> = todos_iter.filter_map(Result::ok).collect();
    println!("todos: {:?}", todos);

    Ok(todos)
}

#[tauri::command]
pub fn change_status(db: State<Db>, status: bool, id: i32) -> Result<bool, String> {
    let conn = match db.0.lock() {
        Ok(c) => c,
        Err(e) => {
            println!("DB lock error: {:?}", e);
            return Err(e.to_string());
        }
    };
    let status_int = if status { 1 } else { 0 };
    println!("todo: {} change status: {}", id, status_int);
    match conn.execute("UPDATE todos SET is_done = ?1 WHERE id = ?2", [status_int, id]) {
        Ok(_) => {
            println!("Updated {}.", id);
            Ok(true)
        },
        Err(err) => {
            println!("Error: {:?}", err);
            Err(format!("Error: {:?}", err))
        }
    }
}
