use std::rc::Rc;
use handlebars::Handlebars;
use ratatui::{layout::Rect, Frame};

use structs::app::AppState;
use components::{
    xml,
    utils::props,
    parsing::xml::parse,
};

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
    
    match xml::create_element(dom_root) {
        xml::El::Paragraph(p) => { 
            frame.render_widget(p, area.unwrap());
        },
        xml::El::Tabs(t) => {
            frame.render_widget(t, area.unwrap());
        },
        xml::El::Layout(l, t_list) => {
            let chunks =  l.split(area.unwrap());
            t_list.into_iter().enumerate().for_each(|(i, t)| {
                match t {
                    Some(t) => {
                        render(frame, store, reg, Some(chunks[i]), &t);
                    },
                    None => {}
                }
            });
       },
       _ => panic!("XML Parse Error !"),
    };
}