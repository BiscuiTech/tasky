use io::{print_header, print_todos, read_keyboard};
use std::io::{stdin, stdout, Write};
use termion::raw::IntoRawMode;

mod commands;
mod io;
mod todo;

fn main() {
    // Initialize 'em all.
    let mut stdout = stdout().lock().into_raw_mode().unwrap();
    let mut stdin = stdin().lock();
    let mut cursor: u16 = 0;
    print_header(&mut stdout);
    print_todos(&mut stdout, &mut cursor);
    stdout.flush().unwrap();

    loop {
        read_keyboard(&mut stdin, &mut stdout, &mut cursor);
        stdout.flush().unwrap();
    }
}
