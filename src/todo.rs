use std::io;
use std::io::prelude::*;

#[derive(Debug)]
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
    pub fn getTitle(&self) -> String {
        let mut string = String::new();
        string.push_str(&self.title);
        string
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

pub struct Todos {
    todos: Vec<Todo>,
}
impl Todos {
    pub fn getStringPlaceholders(&self) -> String {
        let mut string = String::new();
        for todo in &self.todos {
            string.push_str("{:?}");
        }
        string
    }
    pub fn getTodoTitles(&self) -> Vec<String> {
        let mut titles = Vec::new();
        for todo in &self.todos {
            titles.push(todo.title.clone());
        }
        titles
    }
}
