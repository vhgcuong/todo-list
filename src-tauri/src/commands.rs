use crate::{models::Todo, db::Db};
use tauri::State;
use rusqlite::{Connection, Result};

#[tauri::command]
pub fn todos(db: State<Db>) -> Result<Vec<Todo>, String> {
    let conn = db.0.lock().unwrap();

    let mut stmt = conn.prepare("SELECT id, text, is_done FROM todos")
        .map_err(|e| e.to_string())?;
    // let mut rows = stmt.query([])?;

    // let mut todos = Vec::new();
    //
    // while let Some(row) = rows.next()? {
    //     let id: i32 = row.get(0)?;
    //     let text: String = row.get(1)?;
    //     let is_done: bool = row.get(2)?;
    //     todos.push(Todo {id, text, is_done})
    // }

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

// #[tauri::command]
// pub fn create_todo(todo: &Todo) -> bool {
//     let conn = Connection::open("app_database.db")?;
//
//     match conn.execute("INSERT INTO todos (text, is_done) VALUES (:text, :is_done)",
//                        &[(":text", &todo.text), (":is_done", &todo.is_done.to_string())]) {
//         Ok(_) => {
//             println!("Created.");
//             true
//         }
//         Err(err) => {
//             println!("Error: {:?}", err);
//             false
//         }
//     }
// }

#[tauri::command]
pub fn change_done(db: State<Db>, is_done: bool, id: i32) -> bool {
    let conn = db.0.lock().unwrap();
    let is_done_int = if is_done { 1 } else { 0 };
    match conn.execute("UPDATE todos SET is_done = ?1 WHERE id = ?2",
                       [is_done_int, id]) {
        Ok(_) => {
            println!("Updated.");
            true
        }
        Err(err) => {
            println!("Error: {:?}", err);
            false
        }
    }
}
