extern crate pancurses;

use pancurses::{ALL_MOUSE_EVENTS, endwin, getmouse, initscr, mousemask, Input};

fn main() {
    let window = initscr();

    window.keypad(true); // Set keypad mode
    mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut()); // Listen to all mouse events

    window.printw("Click in the terminal, press q to exit\n");
    window.refresh();

    loop {
        match window.getch() {
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    window.mvprintw(1, 0,
                                    &format!("Mouse at {},{}", mouse_event.x, mouse_event.y),
                    );
                };
            }
            Some(Input::Character(x)) if x == 'q' => break,
            _ => (),
        }
    }
    endwin();
}