#[derive(Debug)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub is_done: bool
}

impl Todo {
    pub fn set(id: i32, text: &str, is_done: bool) -> Self {
        Self {
            id,
            text: text.to_string(),
            is_done,
        }
    }

    pub fn mark_done(&mut self) {
        self.is_done = true;
    }
}
