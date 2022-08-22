use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Todo {
    title: String,
    completed: bool,
    // id: u32,
}
const CHECKED: char = '◉';
const NOT_CHECKED: char = '◯';
const ARROW: char = '→';

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
    pub fn rename(&mut self, new_title: String) {
        self.title = new_title;
    }
    pub fn print(&self) -> String {
        let completed = if self.completed { CHECKED } else { NOT_CHECKED };
        let string = completed.to_string() + " " + &self.title + "\n";
        // io::stdout().write_all(string.as_bytes()).unwrap();
        return string;
    }
    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
    pub fn set_complete(&mut self) {
        self.completed = true;
    }
    pub fn set_incomplete(&mut self) {
        self.completed = false;
    }
}

pub struct Todos {
    todos: Vec<Todo>,
}
impl Todos {
    pub fn new() -> Todos {
        Todos { todos: Vec::new() }
    }
    pub fn push(&mut self, todo: Todo) {
        self.todos.push(todo);
    }
    pub fn print(&self, position: u16) -> String {
        let mut y: u16 = 3;
        let x: u16 = 1;
        // iterator over todos
        let mut payload = String::new();
        for (i, todo) in self.todos.iter().enumerate() {
            // \x1B[{};{}H  replace {} with positionnal arguments
            let string = "\x1B[{};{}H{} {}";
            // append to string
            let cursor = match usize::from(position) {
                i => String::from(ARROW),
                _ => String::new(),
            };
            payload.push_str(
                string
                    .replacen("{}", y.to_string().as_str(), 1)
                    .replacen("{}", x.to_string().as_str(), 1)
                    .replacen("{}", cursor.as_str(), 1)
                    .replacen("{}", todo.print().as_str(), 1)
                    .as_str(),
            );
            // increment y
            y += 1;
        }
        payload
    }
    pub fn getTodoTitles(&self) -> Vec<String> {
        let mut titles = Vec::new();
        for todo in &self.todos {
            titles.push(todo.title.clone());
        }
        titles
    }
}
