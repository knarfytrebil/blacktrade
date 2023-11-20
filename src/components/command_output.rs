use serde_json::{Value, json};
use ratatui::layout::Rect;

pub fn template() -> String {
    String::from(r#"
        <Paragraph 
            styles='{"fg": "cyan", "bg": "reset"}'
            wrap='{"trim": true}'
            alignment='{"position": "Left"}'>
            {{#height_buffer props.console_output_lines as |lines|}}
                {{#each lines as |line| ~}}
                    <Line>
                        <Span>{{line}}</Span>
                    </Line>
                {{/each}}
            {{/height_buffer}}
        </Paragraph>"#
    )
}

pub fn props(store: &Value, area: Rect) -> Value {
   json!({
        "props": {
            "console_output_lines": store["console_output_lines"],
            "area": {
                "height": area.height,
                "width": area.width
            }
        }
    })
}