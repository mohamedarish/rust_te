pub mod document;
pub mod editor;
pub mod filetype;
pub mod highlighting;
pub mod rows;
pub mod terminal;
pub use document::Document;
pub use editor::Position;
pub use editor::SearchDirection;
pub use filetype::FileType;
pub use filetype::HighlightingOptions;
pub use rows::Row;
pub use terminal::Terminal;

// macro_export]
// macro_rules! display_on_screen {
//     () => {
//         println!("{}{}", color::Bg(color::Blue), style::Reset)
//     };
//     ($fmt:expr) => {
//         println!("{}{$fmt}{}", color::Bg(color::Blue), style::Reset)
//     };
//     ($fmt:expr, $($arg:tt)+) => {
//         println!("{}{}{}{}", color::Bg(color::Blue), $fmt, $($arg)+, style::Reset)
//     };
// }
//
