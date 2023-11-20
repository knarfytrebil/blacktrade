use serde_json::Value;
use ratatui::layout::Rect;
use ratatui::Frame;
use handlebars::Handlebars;

use components::command_bar;
use components::command_output;
use components::status_bar;
use components::tabs;
use components::xml;
use components::parsing::xml::parse;
use structs::app::AppState;


pub fn render_component(
    frame: &mut Frame, 
    store: &AppState, 
    reg: &mut Handlebars<'_>,
    area: Rect, 
    template: fn() -> String,
    props: fn(&Value, Rect)-> Value
) {
    let dom_root = parse(
        template(),
        Some(&props(&store.json_store, area)),
        reg
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

pub fn render(
    frame: &mut Frame,
    store: &AppState,
    reg: &mut Handlebars<'_>
) {
    let dom_root = parse(
        TEMPLATE.to_string(), 
        None,
        reg
    );
    
    let chunks = match xml::create_element(dom_root) {
        xml::El::Layout(l) => l.split(frame.size()),
        _ => panic!("XML Parse Error !"),
    };

    render_component(frame, store, reg, chunks[0], tabs::template, tabs::props);
    render_component(frame, store, reg, chunks[2], status_bar::template, status_bar::props);
    render_component(frame, store, reg, chunks[3], command_bar::template, command_bar::props);
    match store.tabs.selection {
        0 => render_component(frame, store, reg, chunks[1], command_output::template, command_output::props),
        1 => {}
        _ => {}
    }

}

const TEMPLATE: &'static str = r#"
<Layout direction='vertical'>
    <Constraint
        view='basic_tabs'
        type='{"length":1}'
    />
    <Constraint
        view='CommandOutput'
        type='{"min":1}'
    />
    <Constraint
        view='status_bar'
        type='{"length":1}'
    />
    <Constraint
        view='CommandBar'
        type='{"length":1}'
    />
</Layout>
"#;