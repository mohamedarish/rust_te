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
}
