pub mod editor;
pub mod terminal;

use std::io::{stdin, Error};

use termion::{clear, event::Key, input::TermRead};

pub fn die(e: Error) {
    clear_screen();
    panic!("{e}\nExiting...");
}

pub fn clear_screen() {
    println!("{}", clear::All);
}

pub fn read_key() -> Key {
    loop {
        if let Some(key) = stdin().lock().keys().next() {
            return key.expect("Cannot parse key");
        }
    }
}
