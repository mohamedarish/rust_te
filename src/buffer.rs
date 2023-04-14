use crate::document::Document;

pub struct Buffer {
    content: Vec<Rows>,
}

impl Buffer {
    pub fn new(document: Document, lines: u16, width: u16, start: u16) -> Self {
        Self {
            content: document.content[start..start + lines],
        }
    }
}
