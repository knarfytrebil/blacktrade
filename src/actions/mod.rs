pub mod command;
// use reducers::CommandGen;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use structs::app::events;
use structs::app::events::Key;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppAction {
    Keyboard(Key),
    CommandInvalid(String),
    CommandCreate(String),
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
    SetMode(Value),
}

impl AppAction {
    pub fn into_event(self) -> events::Event {
        events::Event::Dispatch(self)
    }
}
