extern crate serde;
use serde::{Deserialize, Serialize};

// Cargoed from termion 1.5.5
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    /// Backspace.
    Backspace,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Backward Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    F(u8),
    /// Normal character.
    Char(char),
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    ///
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Null byte.
    Null,
    /// Esc key.
    Esc,

    #[doc(hidden)]
    __IsNotComplete,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ModeCategory {
    Normal,
    Command,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppMode {
    pub category: ModeCategory,
    pub symbol: String,
}

impl AppMode {
    pub fn get_mode(mode_name: &str) -> AppMode {
        match mode_name {
            "normal" => AppMode {
                category: ModeCategory::Normal,
                symbol: String::from("NORM"),
            },
            "command" => AppMode {
                category: ModeCategory::Command,
                symbol: String::from("CTRL"),
            },
            &_ => AppMode {
                category: ModeCategory::Command,
                symbol: String::from("Unknown Mode"),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppAction {
    Keyboard(Key),
    CommandInvalid(String),
    CommandCreate(String),
    //    CommandRun {
    //        func: CommandGen,
    //        uuid: String,
    //    },
    CommandEnd {
        uuid: String,
        success: bool,
        reason: String,
    },
    CommandConsume(String),
    CommandBarPush(char),
    CommandBarPop(u16),
    CommandBarSet(String),
    CommandBarEnqueueCmd(String),
    ConsolePush(String),
    SetMode(AppMode),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KeySettingItem {
    key: Key,
    action: AppAction,
}

fn main() -> Result<(), serde_yaml::Error> {
    let point = KeySettingItem {
        key: Key::Char(':'),
        action: AppAction::SetMode(AppMode::get_mode("command")),
    };

    let s = serde_yaml::to_string(&point)?;
    // assert_eq!(s, "---\nx: 1.0\ny: 2.0");
    println!("{}", s);

    // let deserialized_point: KeySettingItem = serde_yaml::from_str(&s)?;
    // assert_eq!(point, deserialized_point);
    Ok(())
}
