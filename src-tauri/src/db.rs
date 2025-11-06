use rusqlite::Connection;
use std::sync::Mutex;

pub struct Db(pub Mutex<Connection>);
