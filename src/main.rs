#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_or_else_default,
    clippy::unwrap_used
)]

use rust_te::editor::Editor;

fn main() {
    Editor::default().run();
}
