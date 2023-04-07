use crate::{command::Command, display_error, display_warning};
use ropey::Rope;
use std::{fs::File, io::Write, ops::ControlFlow, path::PathBuf};

pub struct Editor {
    text: Rope,
    path: Option<PathBuf>,
    has_unsaved_changes: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            text: Rope::new(),
            path: None,
            has_unsaved_changes: false,
        }
    }

    pub fn open(&mut self, path: PathBuf) {
        if self.has_unsaved_changes {
            display_error("current file has unsaved changes");
            return;
        }

        let Ok(text) = File::open(&path).and_then(Rope::from_reader) else {
            display_error("failed to open file");
            return;
        };
        self.text = text;
        self.path = Some(path);
    }

    pub fn read_and_run_command(&mut self) -> ControlFlow<()> {
        print!("> ");
        std::io::stdout().flush().ok();
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            return ControlFlow::Continue(());
        }
        let input = input.trim();
        if input.is_empty() {
            return ControlFlow::Continue(());
        }

        let command: Command = match input.parse() {
            Ok(command) => command,
            Err(err) => {
                display_error(err);
                return ControlFlow::Continue(());
            }
        };
        self.run_command(&command)
    }

    fn run_command(&mut self, command: &Command) -> ControlFlow<()> {
        match *command {
            Command::Quit { force } => {
                if self.has_unsaved_changes {
                    if force {
                        display_warning("discarding unsaved changes");
                    } else {
                        display_error("current file has unsaved changes");
                        return ControlFlow::Continue(());
                    }
                }
                ControlFlow::Break(())
            }
            Command::Write { quit } => {
                let Some(path) = &self.path else {
                    display_error("no path provided");
                    return ControlFlow::Continue(())
                };
                if File::create(path)
                    .and_then(|file| self.text.write_to(file))
                    .is_err()
                {
                    display_error("failed to write file");
                    return ControlFlow::Continue(());
                }

                self.has_unsaved_changes = false;

                if quit {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            }
            Command::Print => {
                print!("{}", self.text);
                ControlFlow::Continue(())
            }
            Command::Info => {
                let lines = self.text.len_lines();
                let code_points = self.text.len_chars();
                let bytes = self.text.len_bytes();
                if let Some(path) = &self.path {
                    print!("{}: ", path.display());
                }
                let all_ascii = if code_points == bytes {
                    ", all ASCII"
                } else {
                    ""
                };
                println!(
                    "{lines} line{}, \
                     {code_points} code point{}, \
                     {bytes} byte{}{all_ascii}",
                    plural_s(lines),
                    plural_s(code_points),
                    plural_s(bytes),
                );
                ControlFlow::Continue(())
            }
        }
    }
}

const fn plural_s(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}
