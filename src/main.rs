use io::{print_header, print_todos, read_keyboard, read_todos_file, CommandInterface};
use std::io::{stdin, stdout, StdinLock, Write};
use termion::raw::IntoRawMode;

mod commands;
mod io;
mod todo;

fn main() {
    // Initialize 'em all.
    let mut stdout = stdout().lock().into_raw_mode().unwrap();
    let mut stdin: StdinLock<'static> = stdin().lock();
    let mut cursor: u16 = 0;
    let todos = read_todos_file();
    let mut commandInterface = CommandInterface {
        stdin,
        stdout,
        cursor,
        todos,
    };
    print_header(&mut commandInterface);
    print_todos(&mut commandInterface);

    loop {
        read_keyboard(&mut commandInterface);
    }
}
