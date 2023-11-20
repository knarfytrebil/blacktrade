use ratatui::layout::Rect;
use ratatui::Frame;
use handlebars::Handlebars;

use components::xml;
use components::parsing::xml::parse;
use components::utils::props;
use structs::app::AppState;


pub fn render_component<'a>(
    frame: &mut Frame, 
    store: &AppState, 
    reg: &mut Handlebars<'_>,
    area: Option<Rect>, 
    template: &'a str,
) {
    let dom_root = parse(
        template,
        &props(&store.json_store, area),
        reg
    );

    match xml::create_element(dom_root) {
        xml::El::Paragraph(p) => frame.render_widget(p, area.unwrap()),
        xml::El::Tabs(t) => frame.render_widget(t, area.unwrap()),
        xml::El::Layout(l) => {
            l.split(frame.size());
        },
        _ => panic!("XML Parse Error !"),
    };
}

pub fn render<'a>(
    frame: &mut Frame,
    store: &AppState,
    reg: &mut Handlebars<'_>,
    area: Option<Rect>, 
    template: &'a str,
) {
    let dom_root = parse(
        template, 
        &props(&store.json_store, area),
        reg
    );
    
    let chunks = match xml::create_element(dom_root) {
        xml::El::Layout(l) => l.split(frame.size()),
        _ => panic!("XML Parse Error !"),
    };

    render_component(frame, store, reg, Some(chunks[0]), "tabs");
    render_component(frame, store, reg, Some(chunks[2]), "status_bar");
    render_component(frame, store, reg, Some(chunks[3]), "command_bar");
    match store.tabs.selection {
        0 => render_component(frame, store, reg, Some(chunks[1]), "command_output"),
        1 => {}
        _ => {}
    }

}