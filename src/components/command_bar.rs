use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

use structs::app::AppState;
use components::xml;

const DATA: &'static str = r#"
<Paragraph>
    <Spans>{{command}}</Spans>
</Paragraph>"#;

pub fn render<B>(
    frame: &mut Frame<B>, 
    store: &AppState, 
    area: Rect
)
where
    B: Backend,
{
    let dom_root = xml::parse(
        DATA.to_string(), 
        &store.json_store
    );
    let widget = match xml::create_element(dom_root) {
        xml::El::Paragraph(p) => p,
        _ => panic!("XML Parse Error !")
    };
    frame.render_widget(widget, area);
}
