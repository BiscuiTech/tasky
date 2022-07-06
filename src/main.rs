mod io;
mod todo;
use std::process;

use io::read_data;
use ncurses::{ll::wget_wch, *};

fn main() {
    // call read_data
    let content = read_data();
    // iterate through the vector of Todos
    // for todo in content {
    //    todo.print()
    // }
    /* If your locale env is unicode, you should use `setlocale`. */
    // let locale_conf = LcCategory::all;
    // setlocale(locale_conf, "zh_CN.UTF-8"); // if your locale is like mine(zh_CN.UTF-8).

    /* Start ncurses. */
    initscr();

    /* Print to the back buffer. */
    addstr("Tasky!\n");
    addstr("(q) quit (a) add (r) rename (t) toggle (d) delete\n");
    hline(ACS_HLINE(), COLS());

    /* Print some unicode(Chinese) string. */
    // addstr("Great Firewall dislike VPN protocol.\nGFW 不喜欢 VPN 协议。");

    /* Wait for input. */
    let ch = wget_wch(stdscr());
    match ch {
        Some(WchResult::KeyCode(q)) => {
            process::exit(0);
        }

        // Some(WchResult::KeyCode(_)) => {
        //     /* Enable attributes and output message. */
        //     attron(A_BOLD | A_BLINK);
        //     addstr("\nKeycode");
        //     attroff(A_BOLD | A_BLINK);
        //     addstr(" pressed");
        // }

        // Some(WchResult::Char(c)) => {
        //     /* Enable attributes and output message. */
        //     addstr("\nKey pressed: ");
        //     attron(A_BOLD | A_BLINK);
        //     addstr(format!("{}\n", char::from_u32(c as u32).expect("Invalid char")).as_ref());
        //     attroff(A_BOLD | A_BLINK);
        // }
        None => {
            addstr("\nYou didn't enter a character in time!");
        }
    }

    /* Update the screen. */
    refresh();

    /* Wait for a key press. */
    getch();

    /* Terminate ncurses. */
    endwin();
}
