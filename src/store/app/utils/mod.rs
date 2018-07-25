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
    return "\n\nimport random\n\ndef main():\n    if random.randint(0, 2) == 0:\n        return 1    \n    return sum([i['ask_price'] for i in data])\n";
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
