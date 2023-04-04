use crate::display_error;
use ropey::Rope;
use std::{fs::File, io::Write, path::Path};

pub struct Editor {
    text: Rope,
}

impl Editor {
    pub fn new() -> Self {
        Self { text: Rope::new() }
    }

    pub fn open(&mut self, path: &Path) {
        let Ok(text) = File::open(path).and_then(Rope::from_reader) else {
            display_error("failed to open file");
            return;
        };
        self.text = text;
    }

    pub fn read_and_run_command(&mut self) {
        print!("> ");
        std::io::stdout().flush().ok();
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            return;
        }
        let input = input.trim();
        match input {
            _ => display_error(format_args!("invalid command: {input}")),
        }
    }
}
