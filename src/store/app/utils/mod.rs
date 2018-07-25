use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;
use store::app::AppMode;
use store::app::AppState;

pub mod python;

#[macro_export]
macro_rules! format_output {
    ($color:expr, $title:expr, $output:expr) => {
        format!("{{fg={} [{}] }} {:?}\n", $color, $title, $output)
    };
}

fn read_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn get_snippet() -> String {
    match read_file(Path::new("example.py")) {
        Ok(code) => code,
        Err(_) => String::from("raise IOError"),
    }
}

impl AppState {
    // helper functions
    pub fn set_mode(&mut self, mode: &str) {
        match mode {
            "command" => {
                self.mode = AppMode::get_mode("command");
                self.command.push(':');
            }
            "normal" => {
                self.mode = AppMode::get_mode("normal");
                self.command.clear();
            }
            _ => {}
        }
    }
}
