pub mod editor;
pub mod terminal;

use std::io::Error;

pub fn die(e: Error) {
    // clear the screen before exiting
    panic!("{e}\nExiting...");
}
