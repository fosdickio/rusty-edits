# Rust Text Editor

![Rust](https://github.com/fosdickio/rust-text-editor/workflows/Rust/badge.svg)

A simple text editor written in Rust.

## Requirements
- Rust (1.49.0+)
- Cargo (1.49.0+)

## Instructions
You can build and run the text editor using the following command:
```rust
cargo run
```

To open a specific file, you may pass in the filename as an argument:
```rust
cargo run README.md
```

## Keyboard Shorcuts
- `Arrow Keys` &rarr; move cursor in the specified direction
- `Page Up` / `Page Down` &rarr; move cursor to the top or bottom of the file
- `Home` / `End` &rarr; move cursor to the start or end of the line
- `Ctrl + q` &rarr; quit the text editor.
