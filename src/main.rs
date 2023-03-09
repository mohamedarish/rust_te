#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_or_else_default,
    clippy::unwrap_used
)]

use std::env;

use rust_te::editor::Editor;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = if args.len() > 1 {
        args[1].to_string()
    } else {
        String::new()
    };

    Editor::default().run(arg);
}
