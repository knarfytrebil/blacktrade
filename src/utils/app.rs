// use std::env;
// use store::app::AppMode;
// use store::app::AppState;
// use utils::fs::read_file;
// pub mod python;

// #[macro_export]
// macro_rules! format_output {
//     ($color:expr, $title:expr, $output:expr) => {
//         format!("{{fg={} [{}] }} {:?}\n", $color, $title, $output)
//     };
// }

// pub fn get_snippet(_filepath: &str) -> String {
//     let config_dir = ".cryptocmd";
//     let base_path = env::home_dir().unwrap().join(config_dir);
//     let snippet_path = base_path.join(_filepath);
//     let _path_str = snippet_path.to_string_lossy();
//     return read_file(&_path_str);
// }

// impl AppState {
//     // helper functions
//     pub fn set_mode(&mut self, mode: &str) {
//         match mode {
//             "command" => {
//                 self.mode = AppMode::get_mode("command");
//                 self.command.push(':');
//             }
//             "normal" => {
//                 self.mode = AppMode::get_mode("normal");
//                 self.command.clear();
//             }
//             _ => {}
//         }
//     }
// }
use termion::event;

pub fn key_event_to_string(event: event::Key) -> String {
    let re = match event {
        event::Key::Backspace => "Backspace",
        event::Key::Left => "Left",
        event::Key::Right => "Right",
        event::Key::Up => "Up",
        event::Key::Down => "Down",
        event::Key::Home => "Home",
        event::Key::End => "End",
        event::Key::PageUp => "PageUp",
        event::Key::PageDown => "PageDown",
        event::Key::BackTab => "BackTab",
        event::Key::Delete => "Delete",
        event::Key::Insert => "Insert",
        event::Key::F(_function_key_num) => &format!("F {}", &_function_key_num.to_string()),
        event::Key::Char(_char) => &format!("CHAR {}", _char),
        event::Key::Alt(_char) => &format!("ALT {}", _char),
        event::Key::Ctrl(_char) => &format!("CTRL {}", _char),
        event::Key::Null => "Null",
        event::Key::Esc => "Esc",
    };
    re.to_string()
}

fn is_functional(event_string: String) -> bool {
    event_string.starts_with("CHAR ")
        || event_string.starts_with("ALT ")
        || event_string.starts_with("CTRL ")
        || event_string.starts_with("F ")
}

pub fn string_to_key_event(event_string: String) -> event::Key {
    match is_functional(event_string) {
        true => {
            if event_string.starts_with("CHAR ") {
                let extracted: Vec<&str> = event_string.split("CHAR ").collect();
                event::Key::Char(extracted[1].chars().next().expect("String is Empty"))
            } else if event_string.starts_with("ALT ") {
                let extracted: Vec<&str> = event_string.split("ALT ").collect();
                event::Key::Alt(extracted[1].chars().next().expect("String is Empty"))
            } else if event_string.starts_with("CTRL ") {
                let extracted: Vec<&str> = event_string.split("CTRL ").collect();
                event::Key::Ctrl(extracted[1].chars().next().expect("String is Empty"))
            } else if event_string.starts_with("F ") {
                let extracted: Vec<&str> = event_string.split("F ").collect();
                event::Key::F(extracted[1].parse::<u8>().unwrap())
            } else {
                panic!("Data is corrupted !");
            }
        }
        false => match event_string.as_str() {
            "Backspace" => event::Key::Backspace,
            "Left" => event::Key::Left,
            "Right" => event::Key::Right,
            "Up" => event::Key::Up,
            "Down" => event::Key::Down,
            "Home" => event::Key::Home,
            "End" => event::Key::End,
            "PageUp" => event::Key::PageUp,
            "PageDown" => event::Key::PageDown,
            "BackTab" => event::Key::BackTab,
            "Delete" => event::Key::Delete,
            "Insert" => event::Key::Insert,
            "Null" => event::Key::Null,
            "Esc" => event::Key::Esc,
        },
    }
}
