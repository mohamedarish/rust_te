use termion::event::Key;

use crate::{
    clear_screen,
    document::Document,
    read_key,
    rows::Rows,
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
                self.cursor_position.y += 1;
                self.document.content.push(Rows::from(""));
                self.process_movement(Key::Home);
            }
            Key::Char(c) => {
                self.handle_character_entered(c);
            }
            Key::Backspace => {
                self.handle_backspace();
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

        let width = self.terminal.width();
        let line_width = self.document.content[y as usize].number_of_characters() as u16;
        let height = self.document.length() as u16;

        match key {
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < line_width {
                    x = x.saturating_add(1);
                }

                if x >= width {
                    todo!("move line a character to the right");
                }
            }
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if height > 0 && y < height - 1 {
                    y = y.saturating_add(1);
                }
            }
            Key::Home => {
                x = 0;
            }
            Key::End => {
                x = line_width;
            }
            Key::PageUp => {
                y = 0;
            }
            Key::PageDown => {
                y = height;
            }
            _ => todo!(),
        }

        self.cursor_position = Position { x, y };

        self.handle_x_overflow(y);
    }

    fn handle_x_overflow(&mut self, new_line_index: u16) {
        let max_width =
            self.document.content[new_line_index as usize].number_of_characters() as u16;

        if self.cursor_position.x > max_width {
            self.cursor_position.x = max_width;
        }
    }

    fn handle_character_entered(&mut self, c: char) {
        self.document.content[self.cursor_position.y as usize]
            .add_character(self.cursor_position.x as usize, c);

        let old_x = self.cursor_position.x + 1;

        self.cursor_position.x = 0;

        set_cursor_position(self.cursor_position);

        flush();

        print!(
            "{}",
            self.document.content[self.cursor_position.y as usize].content()
        );

        flush();

        self.cursor_position.x = old_x;
    }

    fn handle_backspace(&mut self) {
        if self.cursor_position.x == 0 {
            return;
        }

        self.document.content[self.cursor_position.y as usize]
            .remove_character(self.cursor_position.x as usize - 1);

        let old_x = self.cursor_position.x - 1;

        self.cursor_position.x = 0;

        set_cursor_position(self.cursor_position);

        print!(
            "{}{}",
            termion::clear::CurrentLine,
            self.document.content[self.cursor_position.y as usize].content()
        );

        self.cursor_position.x = old_x;
    }
}
