use termion::event::Key;

use crate::{
    clear_screen, read_key,
    terminal::{flush, set_cursor_position, Terminal},
};

#[derive(Default, Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Default)]
pub struct Editor {
    quit_issued: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn run(&mut self) {
        clear_screen();

        loop {
            set_cursor_position(self.cursor_position);

            flush();

            if self.quit_issued {
                clear_screen();
                break;
            }

            self.process_keypress();
        }
    }

    fn process_keypress(&mut self) {
        let pressed_key = read_key();

        match pressed_key {
            Key::Ctrl('g') => {
                self.quit_issued = true;
            }
            Key::Char(c) => {
                print!("{c}");

                self.cursor_position.x += 1;
            }
            Key::Backspace => {
                self.cursor_position.x -= 1;

                set_cursor_position(self.cursor_position);

                print!(" ");
            }
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::Home
            | Key::End => self.process_movement(pressed_key),
            _ => {}
        }
    }

    fn process_movement(&mut self, key: Key) {
        let terminal_height = self.terminal.size().height;
        let terminal_width = self.terminal.size().width;

        let Position { mut x, mut y } = self.cursor_position;

        match key {
            Key::Left => {
                x -= 1;
            }
            Key::Right => {
                x += 1;
            }
            Key::Up => {
                y -= 1;
            }
            Key::Down => {
                y += 1;
            }
            Key::Home => {
                x = 0;
            }
            Key::End => {
                x = terminal_width;
            }
            Key::PageUp => {
                y = 0;
            }
            Key::PageDown => {
                y = terminal_height;
            }
            _ => todo!(),
        }

        self.cursor_position = Position { x, y };
    }
}
