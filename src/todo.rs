use std::io;
use std::io::prelude::*;

pub struct Todo {
    title: String,
    completed: bool,
    // id: u32,
}

impl Todo {
    pub fn new(title: String, completed: bool) -> Todo {
        Todo {
            title,
            completed,
            // id: id,
        }
    }
    pub fn print(&self) {
        let completed = if self.completed { "[X]" } else { "[ ]" };
        let string = completed.to_string() + " " + &self.title + "\n";
        io::stdout().write_all(string.as_bytes()).unwrap();
    }
    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
    pub fn rename(&mut self, new_title: String) {
        self.title = new_title;
    }
}
