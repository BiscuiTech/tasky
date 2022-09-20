#[derive(Debug)]
pub struct Todo {
    title: String,
    completed: bool,
    // id: u32,
}
pub const CHECKED: char = '◉';
pub const NOT_CHECKED: char = '◯';
pub const ARROW: char = '→';

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
    pub fn format_as_string(&self) -> String {
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
    pub fn get_all(&self) -> Vec<&Todo> {
        self.todos.iter().collect()
    }
    pub fn push(&mut self, todo: Todo) {
        self.todos.push(todo);
    }
    pub fn getTodoTitles(&self) -> Vec<String> {
        let mut titles = Vec::new();
        for todo in &self.todos {
            titles.push(todo.title.clone());
        }
        titles
    }
}
