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
use structs::app::events::Key;
use termion::event;

pub fn to_serializable(event: event::Key) -> Key {
    match event {
        event::Key::Backspace => Key::Backspace,
        event::Key::Left => Key::Left,
        event::Key::Right => Key::Right,
        event::Key::Up => Key::Up,
        event::Key::Down => Key::Down,
        event::Key::Home => Key::Home,
        event::Key::End => Key::End,
        event::Key::PageUp => Key::PageUp,
        event::Key::PageDown => Key::PageDown,
        event::Key::BackTab => Key::BackTab,
        event::Key::Delete => Key::Delete,
        event::Key::Insert => Key::Insert,
        event::Key::F(_fun_num) => Key::F(_fun_num),
        event::Key::Char(_char) => Key::Char(_char),
        event::Key::Alt(_char) => Key::Alt(_char),
        event::Key::Ctrl(_char) => Key::Ctrl(_char),
        event::Key::Null => Key::Null,
        event::Key::Esc => Key::Esc,
        event::Key::__IsNotComplete => Key::__IsNotComplete,
    }
}

pub fn to_unserializable(event: Key) -> event::Key {
    match event {
        Key::Backspace => event::Key::Backspace,
        Key::Left => event::Key::Left,
        Key::Right => event::Key::Right,
        Key::Up => event::Key::Up,
        Key::Down => event::Key::Down,
        Key::Home => event::Key::Home,
        Key::End => event::Key::End,
        Key::PageUp => event::Key::PageUp,
        Key::PageDown => event::Key::PageDown,
        Key::BackTab => event::Key::BackTab,
        Key::Delete => event::Key::Delete,
        Key::Insert => event::Key::Insert,
        Key::F(_fun_num) => event::Key::F(_fun_num),
        Key::Char(_char) => event::Key::Char(_char),
        Key::Alt(_char) => event::Key::Alt(_char),
        Key::Ctrl(_char) => event::Key::Ctrl(_char),
        Key::Null => event::Key::Null,
        Key::Esc => event::Key::Esc,
        Key::__IsNotComplete => event::Key::__IsNotComplete,
    }
}
