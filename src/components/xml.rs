use treexml::{Document, Element};

use structs::app::AppState;
use tui::widgets::Paragraph;
use tui::text::Spans;

pub enum El {
    Div(Paragraph<'static>),
    Text(Spans<'static>)
}


pub fn parse_xml(xml: String) -> Element {
    let doc = Document::parse(xml.as_bytes()).unwrap();
    doc.root.unwrap()
}

pub fn create_element(
    el: Element, 
    store: &AppState, 
) -> El {
    let children: Vec<El> = match el.children.len() > 0 {
        true =>  { 
            el.children.into_iter().map(|chd_el| {
                create_element(chd_el, store)
            }).collect()
        },
        false => vec!()
    };

    let this = match el.name.as_str() {
        "Paragraph" => {
            let el_list: Vec<Spans> = match children.len() > 0 {
                true => { 
                    children.into_iter().map(|child| {
                        match child {
                            El::Text(span) => { span },
                            _ => { panic!("Not a Text Node!") }
                        }
                    }).collect()
                },
                false => vec!()
            };
            El::Div(Paragraph::new(el_list))
        },
        "Spans" => { 
            let text = match el.text {
                Some(txt) => txt,
                None => String::from("") 
            };
            El::Text(Spans::from(text))
        },
        &_ => { panic!("Unknown DOM Token") }
    };

    this
}
