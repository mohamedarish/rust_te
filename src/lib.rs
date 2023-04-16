pub mod document;
pub mod editor;
pub mod rows;
pub mod terminal;

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
