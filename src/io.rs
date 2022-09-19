use crate::todo::{Todo, Todos};
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

pub enum KeyboardInput {
    Key(Key),
    Exit,
}

pub fn read_keyboard(stdin: &mut StdinLock, stdout: &mut RawTerminal<StdoutLock>) {
    let mut keys = stdin.keys();
    let c = keys.next().unwrap().unwrap();

    let command = match c {
        // Quit
        Key::Ctrl('c') => KeyboardInput::Exit,
        Key::Up => KeyboardInput::Key(Key::Up),
        Key::Down => KeyboardInput::Key(Key::Down),
        Key::Left => KeyboardInput::Key(Key::Left),
        Key::Right => KeyboardInput::Key(Key::Right),
        // Enter
        Key::Char('\n') => KeyboardInput::Key(Key::Char('\n')),
        // Space
        Key::Char(' ') => KeyboardInput::Key(Key::Char(' ')),
        _ => KeyboardInput::Key(c),
    };
    execute_command(command, stdout)
}

fn execute_command(command: KeyboardInput, stdout: &mut RawTerminal<StdoutLock>) {
    match command {
        KeyboardInput::Exit => {
            let exiting = "Exiting...";
            print(stdout, exiting, PrintOptions::default());
            std::process::exit(0);
        }
        KeyboardInput::Key(Key::Up) => {
            print(
                stdout,
                "Up",
                PrintOptions::new({
                    &PrintOptions {
                        position: { Position { x: 1, y: 1 } },
                        clear: false,
                    }
                }),
            );
        }
        KeyboardInput::Key(Key::Down) => {
            print(stdout, "Down", PrintOptions::default());
        }
        KeyboardInput::Key(Key::Left) => {
            print(stdout, "Left", PrintOptions::default());
        }
        KeyboardInput::Key(Key::Right) => {
            print(stdout, "Right", PrintOptions::default());
        }
        KeyboardInput::Key(Key::Char('\n')) => {
            print(stdout, "Enter", PrintOptions::default());
        }
        KeyboardInput::Key(Key::Char(' ')) => {
            print(stdout, "Space", PrintOptions::default());
        }
        _ => (),
    }
}

struct PrintOptions {
    position: Position,
    clear: bool,
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

fn print(stdout: &mut RawTerminal<StdoutLock>, text: &str, options: PrintOptions) {
    stdout
        .write_fmt(format_args!(
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(options.position.y, options.position.x),
            text
        ))
        .unwrap();
}

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

fn print_line(stdout: RawTerminal<StdoutLock>, text: str) {}
