use crate::io::print_todos;
use crate::io::{print, Position, PrintOptions};
use std::io::StdoutLock;
use std::io::Write;
use termion::raw::RawTerminal;

pub fn exit(stdout: &mut RawTerminal<StdoutLock>) {
    let exiting = "Exiting...";
    writeln!(
        stdout,
        "{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        exiting,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();
    std::process::exit(0);
}

pub fn move_up(stdout: &mut RawTerminal<StdoutLock>, cursor: &mut u16) {
    if *cursor > 0 {
        *cursor -= 1;
        print_todos(stdout, cursor);
    }
}

pub fn move_down(stdout: &mut RawTerminal<StdoutLock>, cursor: &mut u16) {
    *cursor += 1;
    print_todos(stdout, cursor);
}
