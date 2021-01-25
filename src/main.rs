#![warn(clippy::all, clippy::pedantic)]

use editor::Editor;
pub use editor::Position;
pub use terminal::Terminal;

mod editor;
mod terminal;

fn main() {
    Editor::default().run();
}
