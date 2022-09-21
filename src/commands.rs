use crate::io::{print_todos, CommandInterface};
use std::io::Write;

pub fn exit(interface: &mut CommandInterface) {
    let exiting = "Exiting...";
    writeln!(
        interface.stdout,
        "{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        exiting,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();
    std::process::exit(0);
}

pub fn move_up(interface: &mut CommandInterface) {
    if interface.cursor > 0 {
        interface.cursor -= 1;
        print_todos(interface);
    }
}

pub fn move_down(interface: &mut CommandInterface) {
    interface.cursor += 1;
    print_todos(interface);
}

pub fn delete(interface: &mut CommandInterface) {}
