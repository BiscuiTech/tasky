use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
mod io;
mod todo;
use io::read_data;

fn main() {
    // call read_data
    let content = read_data();
    // iterate through the vector of Todos
    // for todo in content {
    //    todo.print()
    // }

    const HEADER: &str = "Tasky!\n'q' to quit";

    // Initialize 'em all.
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut cursor_index = 2;
    write!(
        stdout,
        "{}{}{}{HEADER}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::style::Bold,
        termion::style::Reset,
        termion::cursor::Goto(cursor_index, 2)
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut keys = stdin.keys();
    loop {
        let c = keys.next().unwrap().unwrap();

        match c {
            // Quit
            Key::Char('q') => return,
            Key::Up => return,
            // Clear the screen
            // Key::Char('c') => write!(stdout, "{}", termion::clear::All),
            _ => write!(
                stdout,
                "{}{}{:?}",
                termion::clear::CurrentLine,
                termion::cursor::Goto(2, 2),
                c
            ),
        }
        .unwrap();

        stdout.flush().unwrap();
    }
}
