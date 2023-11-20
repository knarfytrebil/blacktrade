use serde_json::{Value, json};
use ratatui::layout::Rect;

pub fn template() -> String {
    String::from(r#"<Tabs 
        tabs='{{stringify props.tabs}}'
        styles='{"fg":"grey", "bg":"reset"}'
        highlight_styles='{"fg":"reset", "bg":"white"}'
        divider_styles='{"fg":"white", "bg":"reset"}'  
    />"#)
}

pub fn props(store: &Value, area: Rect) -> Value {
    json!({
        "props": {
            "tabs": store["tabs"],
            "area": {
                "height": area.height,
                "width": area.width
            }
        }
    })
}