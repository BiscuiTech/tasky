#![feature(fmt_internals)]

use std::io::{stdin, stdout, Write};
use termion::raw::IntoRawMode;
mod io;
mod todo;
use io::read_todos_file;

use crate::io::read_keyboard;

fn main() {
    // call read_data
    let todos = read_todos_file();

    const HEADER_1: &str = "Tasky!";
    const HEADER_2: &str = "'ctrl+c' to quit";
    // const CURSOR_MINIMUM_INDEX: u16 = 3;

    // Initialize 'em all.
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = stdin().lock();
    let payload = todos.print(1);
    stdout
        .write_fmt(format_args!(
            // "{}{}{}{HEADER_1}{}{}{}{HEADER_2}{}\x1B[3;1H{todo1}\x1B[4;1H{todo2}",
            "{}{}{}{HEADER_1}{}{}{}{HEADER_2}{}{payload}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::style::Bold,
            // HEADER 1
            termion::style::Reset,
            termion::cursor::Goto(1, 2),
            termion::style::Underline,
            // HEADER 2
            termion::style::Reset,
            // payload
        ))
        .unwrap();
    stdout.write(b"test").unwrap();
    stdout.flush().unwrap();

    loop {
        read_keyboard(&mut stdin, &mut stdout);
        stdout.flush().unwrap();
    }
}

fn getCursorIndex(index: u16) -> u16 {
    const HEADER_SIZE: u16 = 2;
    // return the cursor
    index + HEADER_SIZE
}
