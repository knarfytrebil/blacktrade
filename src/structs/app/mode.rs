use serde::{Deserialize, Serialize};

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
