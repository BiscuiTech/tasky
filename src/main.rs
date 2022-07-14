use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
mod io;
mod todo;
use io::read_data;
use todo::Todo;

fn main() {
    // call read_data
    let content = read_data();

    const HEADER_1: &str = "Tasky!";
    const HEADER_2: &str = "'ctrl+c' to quit";

    // Initialize 'em all.
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut cursor_index = 3;
    let todo1 = content[0].getTitle();
    let todo2 = content[1].getTitle();
    let ite = content.iter();
    stdout
        .write_fmt(format_args!(
            "{}{}{}{HEADER_1}{}{}{}{HEADER_2}{}{}{todo1}{}{todo2}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::style::Bold,
            // HEADER 1
            termion::style::Reset,
            termion::cursor::Goto(1, 2),
            termion::style::Underline,
            // HEADER 2
            termion::style::Reset,
            termion::cursor::Goto(1, 3),
            // Todo 1
            termion::cursor::Goto(1, 4),
            // Todo 1
        ))
        .unwrap();
    stdout.flush().unwrap();

    let mut keys = stdin.keys();
    loop {
        let c = keys.next().unwrap().unwrap();

        match c {
            // Quit
            Key::Ctrl('c') => return,
            Key::Up => (),
            Key::Down => (),
            Key::Left => (),
            Key::Right => (),
            // Enter
            Key::Char('\n') => (),
            // Space
            Key::Char(' ') => (),
            _ => write!(
                stdout,
                "{}{}{:?}",
                termion::clear::CurrentLine,
                termion::cursor::Goto(1, cursor_index),
                c
            )
            .unwrap(),
        }

        stdout.flush().unwrap();
    }
}

fn getCursorIndex(index: u16) -> u16 {
    const HEADER_SIZE: u16 = 2;
    // return the cursor
    index + HEADER_SIZE
}
