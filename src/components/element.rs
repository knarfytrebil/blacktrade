use ratatui::layout::Rect;
use ratatui::Frame;

use components::xml;
use structs::app::AppState;

pub fn render(template: String, frame: &mut Frame, store: &AppState, area: Rect)
{
    let dom_root = xml::parse(template, &store.json_store);
    let widget = match xml::create_element(dom_root) {
        xml::El::Paragraph(p) => p,
        _ => panic!("XML Parse Error !"),
    };
    frame.render_widget(widget, area);
}
