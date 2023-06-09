#![forbid(
    unsafe_code,
    clippy::panic,
    clippy::unwrap_used,
    clippy::indexing_slicing
)]
#![warn(clippy::nursery, clippy::pedantic, clippy::cargo)]

mod command;
mod editor;

use std::{fmt::Display, process::ExitCode};

fn main() -> ExitCode {
    let mut args = std::env::args_os();
    if args.len() > 2 {
        display_error("too many arguments");
        return ExitCode::FAILURE;
    }
    let mut editor = editor::Editor::new();
    if let Some(path) = args.nth(1) {
        editor.open(path.into());
    }

    while editor.read_and_run_command().is_continue() {}

    ExitCode::SUCCESS
}

#[cfg(feature = "color")]
fn display_error(message: impl Display) {
    use owo_colors::{OwoColorize, Stream::Stdout, Style};
    println!(
        "{}: {message}",
        "Error".if_supports_color(Stdout, |text| text
            .style(Style::new().red().bold()))
    );
}

#[cfg(not(feature = "color"))]
fn display_error(message: impl Display) {
    println!("Error: {message}");
}

#[cfg(feature = "color")]
fn display_warning(message: impl Display) {
    use owo_colors::{OwoColorize, Stream::Stdout, Style};
    println!(
        "{}: {message}",
        "Warning".if_supports_color(Stdout, |text| text
            .style(Style::new().yellow().bold()))
    );
}

#[cfg(not(feature = "color"))]
fn display_warning(message: impl Display) {
    println!("Warning: {message}");
}
