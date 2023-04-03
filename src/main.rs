#![forbid(
    unsafe_code,
    clippy::panic,
    clippy::unwrap_used,
    clippy::indexing_slicing
)]
#![warn(clippy::nursery, clippy::pedantic)]

mod editor;

use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let mut args = std::env::args_os();
    if args.len() > 2 {
        display_error("too many arguments");
        return ExitCode::FAILURE;
    }
    let mut editor = editor::Editor::new();
    if let Some(path) = args.nth(1) {
        editor.open(Path::new(&path));
    }

    loop {
        editor.read_and_run_command();
    }
}

fn display_error(message: &str) {
    println!("Error: {message}");
}
