use crate::{
    commands::{exit, move_down, move_up},
    todo::{Todo, Todos, ARROW},
};
use std::{
    fs,
    io::{StdinLock, StdoutLock, Write},
};
use termion::{event::Key, input::TermRead, raw::RawTerminal};

const FILE_NAME: &str = "todos.data";

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 1, y: 1 }
    }
}

pub struct PrintOptions {
    pub position: Position,
    pub clear: bool,
    // pub underline: bool,
    // pub cursor: bool,
}

impl PrintOptions {
    fn new(&self) -> PrintOptions {
        PrintOptions {
            position: {
                Position {
                    x: self.position.x,
                    y: self.position.y,
                }
            },
            clear: self.clear,
            // underline: self.underline,
        }
    }
}

impl Default for PrintOptions {
    fn default() -> Self {
        PrintOptions {
            position: Position { x: 1, y: 1 },
            clear: false,
        }
    }
}

pub fn read_todos_file() -> Todos {
    // read file todos.data
    let contents = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    // split contents into lines
    let lines = contents.lines();
    // iterate over lines
    let mut todos: Todos = Todos::new();
    for line in lines {
        // split line into parts
        let parts: Vec<&str> = line.split('|').collect();
        // create new todo
        let todo = Todo::new(
            parts[0].to_string(),
            parts[1].to_string().parse::<bool>().unwrap(),
        );
        // add todo to todos
        todos.push(todo);
    }
    // return todos
    todos
}

pub fn read_keyboard(
    stdin: &mut StdinLock,
    stdout: &mut RawTerminal<StdoutLock>,
    cursor: &mut u16,
) {
    let mut keys = stdin.keys();
    let c = keys.next().unwrap().unwrap();

    match c {
        // Quit
        Key::Ctrl('c') => exit(stdout),
        // Navigate
        Key::Up => move_up(stdout, cursor),
        Key::Down => move_down(stdout, cursor),
        Key::Left => (),
        Key::Right => (),
        // Enter
        Key::Char('\n') => (),
        // Space
        Key::Char(' ') => (),
        _ => (),
    };
}

pub fn print(stdout: &mut RawTerminal<StdoutLock>, text: &str, options: PrintOptions) {
    writeln!(
        stdout,
        "{}{}",
        termion::cursor::Goto(options.position.y, options.position.x),
        text
    )
    .unwrap();
}

pub fn print_header(stdout: &mut RawTerminal<StdoutLock>) {
    const HEADER_1: &str = "Tasky!";
    const HEADER_2: &str = "'ctrl+c' to quit";
    // "{}{}{}{HEADER_1}{}{}{}{HEADER_2}{}\x1B[3;1H{todo1}\x1B[4;1H{todo2}",
    writeln!(
        stdout,
        "{}{}{}{HEADER_1}{}{}{}{HEADER_2}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::style::Bold,
        // HEADER 1
        termion::style::Reset,
        termion::cursor::Goto(1, 2),
        termion::style::Underline,
        // HEADER 2
        termion::style::Reset,
    )
    .unwrap();
}

pub fn print_todos(stdout: &mut RawTerminal<StdoutLock>, cursor: &mut u16) {
    // call read_data
    let todos = read_todos_file();
    let mut y: u16 = 3;
    let x: u16 = 1;
    // iterator over todos
    let mut payload = String::new();
    for (i, todo) in todos.get_all().iter().enumerate() {
        // \x1B[{};{}H  replace {} with positionnal arguments
        let string = "\x1B[{};{}H{} {}";
        // append to string
        let cursor = match usize::from(*cursor) == i {
            true => String::from(ARROW),
            false => String::from(" "),
        };
        payload.push_str(
            string
                .replacen("{}", y.to_string().as_str(), 1)
                .replacen("{}", x.to_string().as_str(), 1)
                .replacen("{}", cursor.as_str(), 1)
                .replacen("{}", todo.format_as_string().as_str(), 1)
                .as_str(),
        );
        // increment y
        y += 1;
    }

    writeln!(stdout, "{payload}").unwrap();
}

/*

fn print_2(stdout: &mut RawTerminal<StdoutLock>, text: &str, options: PrintOptions) {
    stdout
        .write_fmt(::core::fmt::Arguments::new_v1(
            &["", "", ""],
            &[
                ::core::fmt::ArgumentV1::new_display(&termion::clear::All),
                ::core::fmt::ArgumentV1::new_display(&termion::cursor::Goto(
                    options.position.y,
                    options.position.x,
                )),
                ::core::fmt::ArgumentV1::new_display(&text),
            ],
        ))
        .unwrap();
}


*/
