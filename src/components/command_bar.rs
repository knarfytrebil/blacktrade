use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

use structs::app::AppState;
use handlebars::Handlebars;
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
    let reg = Handlebars::new();
    let data = reg
        .render_template(DATA, &store.json_store)
        .expect("Template Parse Error");

    let dom_root = xml::parse_xml(data);
    let widget = match xml::create_element(
        dom_root, 
        store
    ) {
        xml::El::Div(p) => p,
        _ => panic!("XML Parse Error !")
    };

    frame.render_widget(widget, area);
}
