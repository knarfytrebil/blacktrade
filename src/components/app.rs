use components::command_bar;
use components::command_output;
use components::status_bar;
use components::tabs;
use components::xml;
use structs::app::AppState;
use serde_json::Value;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

pub fn p_render(
    frame: &mut Frame, 
    store: &AppState, 
    area: Rect, 
    template: fn() -> String,
    props: fn(&Value, Rect)-> Value
) {
    let dom_root = xml::parse(
        template(),
        &props(&store.json_store, area),
    );

    match xml::create_element(dom_root) {
        xml::El::Paragraph(p) => frame.render_widget(p, area),
        xml::El::Tabs(t) => frame.render_widget(t, area),
        xml::El::Layout(l) => {
            l.split(frame.size());
        },
        _ => panic!("XML Parse Error !"),
    };
}

const TEMPLATE: &'static str = r#"
<Layout direction="vertical"/>
    <Constraint type='{"length": "1"}'>
        <Tabs />
    </Constraint>
    <Constraint type='{"min": "1"}'/>
        <CommandOutput />
    </Constraint>
    <Constraint type='{"length": "1"}'/>
        <StatusBar />
    </Constraint>
    <Constraint type='{"length": "1"}'/>
        <CommandBar />
    </Constraint>
</Layout>
"#;

pub fn render(frame: &mut Frame, store: &AppState)
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(frame.size());

    p_render(frame, store, chunks[0], tabs::template, tabs::props);
    p_render(frame, store, chunks[2], status_bar::template, status_bar::props);
    p_render(frame, store, chunks[3], command_bar::template, command_bar::props);
    match store.tabs.selection {
        0 => p_render(frame, store, chunks[1], command_output::template, command_output::props),
        1 => {}
        _ => {}
    }
}
