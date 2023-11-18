use serde_json::{Value, json};
use ratatui::layout::Rect;

pub fn template() -> String {
    String::from(r#"
        <Paragraph>
            <Line>{{props.command}}</Line>
        </Paragraph>"#
    )
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