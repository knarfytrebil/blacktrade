use store::app::AppMode;
use store::app::AppState;

pub mod python;

#[macro_export]
macro_rules! format_output {
    ($color:expr, $title:expr, $output:expr) => {
        format!("{{fg={} [{}] }} {:?}\n", $color, $title, $output)
    };
}

pub fn get_snippet() -> &'static str {
    return 
"
import random
def main():
    if random.randint(0, 2) == 0:
        return 1
    return sum([i['ask_price'] for i in data])
";
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
