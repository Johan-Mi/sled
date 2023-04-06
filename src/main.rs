#![forbid(
    unsafe_code,
    clippy::panic,
    clippy::unwrap_used,
    clippy::indexing_slicing
)]
#![warn(clippy::nursery, clippy::pedantic)]

mod command;
mod editor;

use std::{fmt::Display, path::Path, process::ExitCode};

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

    while editor.read_and_run_command().is_continue() {}

    ExitCode::SUCCESS
}

fn display_error(message: impl Display) {
    println!("Error: {message}");
}

fn display_warning(message: impl Display) {
    println!("Warning: {message}");
}
