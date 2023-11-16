use serde_json::{Value, json};
use components::ele::powerline_symbol as PowerlineSym;
use ratatui::layout::Rect;

const TEMPLATE: &'static str = r#"
<Paragraph>
    <Line>
        <Span styles='{"fg": "white", "bg": "black"}'> </Span>
        <Span styles='{"bg": "black"}'> {{props.mode}} </Span>
        <Span styles='{"fg": "white", "bg": "black"}'> </Span>
        <Span styles='{"fg": "black", "bg": "white"}'>{{props.divider}}</Span>
        <Span styles='{"fg": "black", "bg": "white"}'> Running Tasks </Span>
        <Span styles='{"fg": "black", "bg": "white"}'> </Span>
        <Span styles='{"fg": "white", "bg": "black"}'>{{props.divider}}</Span>
    </Line>
</Paragraph>"#;

pub fn template() -> String {
    TEMPLATE.to_string()
}

pub fn props(store: &Value, area: Rect) -> Value {
    json!({
        "props": {
            "mode": store["mode"]["symbol"],
            "divider": PowerlineSym::RIGHT_ARROW,
            "area": {
                "height": area.height,
                "width": area.width
            }
        }
    })
}