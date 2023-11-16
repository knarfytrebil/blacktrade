use serde_json::{Value, json};
use ratatui::layout::Rect;

const TEMPLATE: &'static str = r#"
<Paragraph>
    <Line>{{props.command}}</Line>
</Paragraph>"#;

pub fn template() -> String {
    TEMPLATE.to_string()
}

pub fn props(store: &Value, area: Rect) -> Value {
    json!({
        "props": {
            "command": store["command"],
            "area": {
                "height": area.height,
                "width": area.width
            }
        }
    })
}