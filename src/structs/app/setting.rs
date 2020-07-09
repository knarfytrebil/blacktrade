use actions::AppAction;
use serde::{Deserialize, Serialize};
use structs::app::events::Key as SerializableKey;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KeySettingItem {
    key: SerializableKey,
    action: AppAction,
}
