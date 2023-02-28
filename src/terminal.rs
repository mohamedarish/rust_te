use std::io::{stdout, Write};

use termion::{
    cursor::Goto,
    raw::{IntoRawMode, RawTerminal},
};

use crate::editor::Position;

#[derive(Clone, Copy)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Default for Terminal {
    fn default() -> Self {
        let size = termion::terminal_size().expect("Cannot get the size of the terminal");

        Self {
            size: Size {
                height: size.0,
                width: size.1,
            },
            _stdout: stdout()
                .into_raw_mode()
                .expect("Cannot parse the stdout interface"),
        }
    }
}

impl Terminal {
    pub fn size(&self) -> Size {
        self.size
    }
}

pub fn flush() {
    stdout().flush().expect("Cannot flush the buffer")
}

pub fn set_cursor_position(position: Position) {
    let Position { mut x, mut y } = position;
    x = x.saturating_add(1);
    y = y.saturating_add(1);

    let x = x;
    let y = y;

    print!("{}", Goto(x, y));
}
