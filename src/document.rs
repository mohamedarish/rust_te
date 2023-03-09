use std::fs;

use crate::rows::Rows;

#[derive(Default)]
pub struct Document {
    filename: String,
    pub content: Vec<Rows>,
}

impl Document {
    pub fn open(filename: &String) -> Self {
        let content = if !filename.is_empty() {
            fs::read_to_string(filename).expect("Does not have permission to access the file")
        } else {
            String::new()
        };

        let mut rows = vec![];

        for line in content.lines() {
            rows.push(Rows::from(line));
        }

        Self {
            filename: filename.to_string(),
            content: rows,
        }
    }

    pub fn filename(&self) -> String {
        self.filename.to_string()
    }

    pub fn length(&self) -> usize {
        self.content.len()
    }
}
