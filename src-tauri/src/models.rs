use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub is_done: bool,
}

impl Todo {
    pub fn new(text: &str, is_done: &bool) -> Self {
        Self {
            id: 0,
            text: text.to_string(),
            is_done: *is_done,
        }
    }
    pub fn mark_done(&mut self) {
        self.is_done = true;
    }
}
