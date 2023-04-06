use crate::{command::Command, display_error};
use ropey::Rope;
use std::{fs::File, io::Write, ops::ControlFlow, path::Path};

pub struct Editor {
    text: Rope,
    has_unsaved_changes: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            text: Rope::new(),
            has_unsaved_changes: false,
        }
    }

    pub fn open(&mut self, path: &Path) {
        if self.has_unsaved_changes {
            display_error("current file has unsaved changes");
            return;
        }

        let Ok(text) = File::open(path).and_then(Rope::from_reader) else {
            display_error("failed to open file");
            return;
        };
        self.text = text;
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
        match command {
            Command::Quit => {
                if self.has_unsaved_changes {
                    display_error("current file has unsaved changes");
                    ControlFlow::Continue(())
                } else {
                    ControlFlow::Break(())
                }
            }
        }
    }
}
