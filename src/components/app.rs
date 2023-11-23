use std::rc::Rc;
use ratatui::layout::Rect;
use ratatui::Frame;
use handlebars::Handlebars;

use components::xml;
use components::parsing::xml::parse;
use components::utils::props;
use structs::app::AppState;

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
    
    let chunks: Option<Rc<[Rect]>>= match xml::create_element(dom_root) {
        xml::El::Paragraph(p) => { 
            frame.render_widget(p, area.unwrap());
            None
        },
        xml::El::Tabs(t) => {
            frame.render_widget(t, area.unwrap());
            None
        },
        xml::El::Layout(l) => {
            Some(l.split(area.unwrap()))
        },
        _ => panic!("XML Parse Error !"),
    };

    if let Some(chunks) = chunks {
        render(frame, store, reg, Some(chunks[0]), "tabs");
        render(frame, store, reg, Some(chunks[2]), "status_bar");
        render(frame, store, reg, Some(chunks[3]), "command_bar");
        match store.tabs.selection {
            0 => render(frame, store, reg, Some(chunks[1]), "command_output"),
            1 => {}
            _ => {}
        }
    }
}