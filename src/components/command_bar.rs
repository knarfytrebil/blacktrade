use serde_json::json;
use ratatui::layout::Rect;
use ratatui::Frame;

use components::xml;
use structs::app::AppState;

const DATA: &'static str = r#"
<Paragraph>
    <Line>{{store.command}}</Line>
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
