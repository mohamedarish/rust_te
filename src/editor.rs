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
        let terminal_height = self.terminal.size().height;
        let terminal_width = self.terminal.size().width;

        let Position { mut x, mut y } = self.cursor_position;

        match key {
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < terminal_width {
                    x = x.saturating_add(1);
                }
            }
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < terminal_height {
                    y = y.saturating_add(1);
                }
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
