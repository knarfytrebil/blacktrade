use serde_json::{Value, json};
use ratatui::layout::Rect;

const DATA: &'static str = r#"
<Paragraph 
    styles='{"fg": "cyan", "bg": "reset"}'
    wrap='{"trim": true}'
    alignment='{"position": "Left"}'>
    {{#each props.console_output_lines as |line| ~}}
        <Line>
            <Span>{{line}}</Span>
        </Line>
    {{/each}}
</Paragraph>"#;

pub fn template() -> String {
    DATA.to_string()
}

pub fn props(store: &Value, area: Rect) -> Value {
    let lines = store["console_output_lines"]
        .as_array().expect("there is nothing");
    let height: usize = area.height.into();
    let output = match lines.len() > height {
        true => lines[lines.len() - height ..].to_vec(),
        false => lines.to_vec()
    };
    json!({
        "props": {
            "console_output_lines": output,
            "area": {
                "height": area.height,
                "width": area.width
            }
        }
    })
}