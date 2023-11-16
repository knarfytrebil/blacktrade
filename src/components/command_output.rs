use serde_json::Value;
use serde_json::json;
use ratatui::layout::Rect;
// use tui::widgets::Block;

// use tui::text::{Span, Spans};
// use tui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

use components::xml;
use structs::app::AppState;

//FIXME: maybe handle scroll properly later scroll='{"offset": [0, 20]}' 
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

fn props(store: &Value, area: Rect) -> Value {
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

pub fn render(frame: &mut Frame, store: &AppState, area: Rect)
{
    let dom_root = xml::parse(
        DATA.to_string(),
        &props(&store.json_store, area),
    );

    let widget = match xml::create_element(dom_root) {
        xml::El::Paragraph(p) => p,
        _ => panic!("XML Parse Error !"),
    };

    frame.render_widget(widget, area);
}