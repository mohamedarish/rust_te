pub mod document;
pub mod editor;
pub mod rows;
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

pub fn read_key() -> Result<Key, Error> {
    loop {
        if let Some(key) = stdin().lock().keys().next() {
            return key;
        }
    }
}

// #[macro_export]
// macro_rules! display_on_screen {
//     () => {
//         println!("{}{}", color::Bg(color::Blue), style::Reset)
//     };
//     ($fmt:expr) => {
//         println!("{}{$fmt}{}", color::Bg(color::Blue), style::Reset)
//     };
//     ($fmt:expr, $($arg:tt)+) => {
//         println!("{}{}{}{}", color::Bg(color::Blue), $fmt, $($arg)+, style::Reset)
//     };
// }
