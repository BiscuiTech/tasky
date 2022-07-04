use crate::todo::Todo;
use std::fs;

const FILE_NAME: &str = "todos.data";

pub fn read_data() -> Vec<Todo> {
  // read file todos.data
  let contents = fs::read_to_string(FILE_NAME)
  .expect("Something went wrong reading the file");
  // split contents into lines
  let lines = contents.lines();
  // iterate over lines
  let mut todos: Vec<Todo> = Vec::new();
  for line in lines {
      // split line into parts
      let parts: Vec<&str> = line.split('|').collect();
      // create new todo
      let todo = Todo::new(
          parts[0].to_string(),
          parts[1].to_string().parse::<bool>().unwrap()
      );
      // add todo to todos
      todos.push(todo);
  }
  // return todos
  todos
}
