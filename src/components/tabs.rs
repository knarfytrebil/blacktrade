use serde_json::{Value, json};
use ratatui::layout::Rect;

const TEMPLATE: &'static str = r#"
<Tabs 
    tabs='{{stringify props.tabs}}'
    styles='{"fg":"grey", "bg":"reset"}'
    highlight_styles='{"fg":"reset", "bg":"white"}'
    divider_styles='{"fg":"white", "bg":"reset"}'  
/>"#;

pub fn template() -> String {
    TEMPLATE.to_string()
}

pub fn props(store: &Value, area: Rect) -> Value {
    debug!("{:?}", store["tabs"]);
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