use structs::app::AppState;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::Paragraph;
use tui::text::Spans;
use tui::Frame;

use components::xml;

const DATA: &'static str = r#"
<Paragraph>
    <Spans> {store.command} </Spans>
</Paragraph>"#;

pub fn render<B>(frame: &mut Frame<B>, store: &AppState, area: Rect)
where
    B: Backend,
{
    let paragraph = Paragraph::new(vec![Spans::from(
        store.command.clone()
    )]);

    // let paragraph = match xml::create_element(xml::parse_xml(DATA)) {
    //     xml::El::Div(p) => p,
    //     _ => panic!("XML Parse Error !")
    // };

    frame.render_widget(paragraph, area);
}
