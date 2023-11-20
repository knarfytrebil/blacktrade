use serde_json::{Value, json};
use ratatui::layout::Rect;

pub fn template() -> String {
    String::from(r#"
        <Paragraph>
            <Line>
                <Span styles='{"fg": "white", "bg": "black"}'> </Span>
                <Span styles='{"bg": "black"}'> {{props.mode}} </Span>
                <Span styles='{"fg": "white", "bg": "black"}'> </Span>
                <Span styles='{"fg": "black", "bg": "white"}'>{{powerline_symbol "right_arrow"}}</Span>
                <Span styles='{"fg": "black", "bg": "white"}'> Running Tasks </Span>
                <Span styles='{"fg": "black", "bg": "white"}'> </Span>
                <Span styles='{"fg": "white", "bg": "black"}'>{{powerline_symbol "right_arrow"}}</Span>
            </Line>
        </Paragraph>"#)
}

pub fn props(store: &Value, area: Rect) -> Value {
    json!({
        "props": {
            "mode": store["mode"]["symbol"],
            "area": {
                "height": area.height,
                "width": area.width
            }
        }
    })
}