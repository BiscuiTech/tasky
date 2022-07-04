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
  pub fn print(&self) -> String {
      // if completed, return "[x] " + title
      let completed = if self.completed { 'x' } else { ' ' };
      format!("[{}] - {}", completed, self.title)
  }
  pub fn toggle(&mut self) {
      self.completed = !self.completed;
  }
  pub fn rename(&mut self, new_title: String) {
      self.title = new_title;
  }
}