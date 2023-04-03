use crate::display_error;
use ropey::Rope;
use std::{fs::File, path::Path};

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
        todo!();
    }
}
