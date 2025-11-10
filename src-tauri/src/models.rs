use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub is_done: bool,
}
