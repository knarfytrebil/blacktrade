use std::env;
use store::app::AppMode;
use store::app::AppState;
use utils::fs::read_file;
pub mod python;

#[macro_export]
macro_rules! format_output {
    ($color:expr, $title:expr, $output:expr) => {
        format!("{{fg={} [{}] }} {:?}\n", $color, $title, $output)
    };
}

pub fn get_snippet(_filepath: &str) -> String {
    let base_path = env::home_dir().unwrap().join(".cryptocmd");
    let snippet_path = base_path.join(_filepath); 
    let _path_str = snippet_path.to_string_lossy();
    return read_file(&_path_str);
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
