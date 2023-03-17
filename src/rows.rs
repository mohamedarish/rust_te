use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Rows {
    content: String,
    length: usize,
}

impl From<&str> for Rows {
    fn from(slice: &str) -> Self {
        Self {
            content: String::from(slice),
            length: slice.graphemes(true).count(),
        }
    }
}

impl Rows {
    pub fn content(&self) -> String {
        self.content.to_string()
    }

    pub fn number_of_words(&self) -> usize {
        self.length
    }

    pub fn number_of_characters(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn add_character(&mut self, position: usize, character: char) {
        self.content.insert(position, character);
    }

    pub fn remove_character(&mut self, position: usize) {
        self.content.remove(position);
    }

    pub fn append(&mut self, character: char) {
        self.content.push(character);
    }
}
