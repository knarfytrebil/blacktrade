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
    styles='{"fg": {"Color": "red"}}' 
    wrap='{"trim": true}' 
    alignment='{"position" : "Left"}'>
    {{#each store.console_output_lines as |line| ~}}
        <Line>
            <Span>{{line}}</Span>
        </Line>
    {{/each}}
</Paragraph>"#;

pub fn render(frame: &mut Frame, store: &AppState, area: Rect)
{
    let dom_root = xml::parse(
        DATA.to_string(),
        &json!({
            "store": &store.json_store,
            "metrics": {
                "height": area.height,
                "width": area.width
            }
        }),
    );

    let widget = match xml::create_element(dom_root) {
        xml::El::Paragraph(p) => p,
        _ => panic!("XML Parse Error !"),
    };

    frame.render_widget(widget, area);
}