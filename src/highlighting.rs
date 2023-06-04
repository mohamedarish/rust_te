use termion::color;
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
}

impl Type {
    pub fn to_color(self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(220, 163, 163),
            Type::Match => color::Rgb(38, 139, 210),
            Type::String => color::Rgb(211, 182, 54),
            Type::Character => color::Rgb(108, 113, 196),
            Type::Comment | Type::MultilineComment => color::Rgb(133, 153, 0),
            Type::PrimaryKeywords => color::Rgb(0, 181, 87),
            Type::SecondaryKeywords => color::Rgb(42, 161, 152),
            _ => color::Rgb(91, 175, 240),
        }
    }
}
