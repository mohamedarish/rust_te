use termion::event::Key;

use crate::{
    clear_screen,
    document::Document,
    read_key,
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
    document: Document,
}

impl Editor {
    pub fn run(&mut self, filename: String) {
        clear_screen();

        self.document = Document::open(&filename);

        set_cursor_position(self.cursor_position);

        // println!("{} {}", self.terminal.height(), self.terminal.width());

        for line in self.document.content.iter() {
            println!("{}\r", line.content());
            flush();
        }

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
            Key::Char('\n') => {
                self.process_movement(Key::Down);
                self.process_movement(Key::Home);
            }
            Key::Char(c) => {
                print!("{c}");

                self.cursor_position.x += 1;
            }
            Key::Backspace => {
                self.process_movement(Key::Left);

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
        let Position { mut x, mut y } = self.cursor_position;

        match key {
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < self.max_x() {
                    x = x.saturating_add(1);
                }
            }
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < self.max_y() - 1 {
                    y = y.saturating_add(1);
                }
            }
            Key::Home => {
                x = 0;
            }
            Key::End => {
                x = self.max_x();
            }
            Key::PageUp => {
                y = 0;
            }
            Key::PageDown => {
                y = self.max_y();
            }
            _ => todo!(),
        }

        self.cursor_position = Position { x, y };

        self.handle_cursor_position_overflow();
    }

    fn handle_cursor_position_overflow(&mut self) {
        if self.cursor_position.x >= self.max_x() {
            self.cursor_position.x = self.max_x();
        }
    }

    fn max_y(&self) -> u16 {
        if self.terminal.height() < self.number_of_rows() {
            self.terminal.height()
        } else {
            self.number_of_rows()
        }
    }

    fn max_x(&self) -> u16 {
        if self.terminal.width() < self.number_of_characters_in_row() {
            self.terminal.width()
        } else {
            self.number_of_characters_in_row()
        }
    }

    fn number_of_characters_in_row(&self) -> u16 {
        self.document.content[self.cursor_position.y as usize]
            .number_of_characters()
            .try_into()
            .expect("Cannot convert into u16")
    }

    fn number_of_rows(&self) -> u16 {
        self.document
            .length()
            .try_into()
            .expect("Cannot convert into u16")
    }
}
