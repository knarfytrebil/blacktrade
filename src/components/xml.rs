use treexml::{Document, Element};
use serde_json::{Value};
use handlebars::Handlebars;
use tui::widgets::Paragraph;
use tui::widgets::Block;
use tui::text::{Spans, Span};

pub enum El {
    Paragraph(Paragraph<'static>),
    Spans(Spans<'static>),
    Span(Span<'static>)
}

pub fn parse_xml(xml: String) -> Element {
    let doc = Document::parse(xml.as_bytes()).unwrap();
    doc.root.unwrap()
}

pub fn parse(template: String, store: &Value) -> Element {
    let reg = Handlebars::new();
    let filled_template = reg
        .render_template(&template, &store)
        .expect("Template Parse Error");
    parse_xml(filled_template)
}

pub fn create_element(
    el: Element, 
) -> El {
    let children: Vec<El> = match el.children.len() > 0 {
        true =>  { 
            el.children
                .into_iter()
                .map(|chd_el| { create_element(chd_el) })
                .collect()
        },
        false => vec!()
    };

    let styles: Option<Value>= match el.attributes.contains_key("styles") {
        true => Some(
            serde_json::from_str(&el.attributes["styles"])
                .expect("JSON Parse Error")
        ),
        false => None
    };

    debug!("STYLES: {:?}", styles);

    let this = match el.name.as_str() {
        "Paragraph" => {
            let el_list: Vec<Spans> = match children.len() > 0 {
                true => { 
                    children.into_iter().map(|child| {
                        match child {
                            El::Spans(s) => { s },
                            _ => { panic!("Not a Text Node!") }
                        }
                    }).collect()
                },
                false => vec!()
            };
            let paragraph_node = Paragraph::new(el_list);
            El::Paragraph(paragraph_node)
            // match styles {
            //     Some(style) => {
            //         match style {
            //             Value::Object(obj) => {
            //                 for (key, value) in obj.iter() {
            //                     match key.as_str() {
            //                         "block" => {
            //                             match value.as_str().expect("Unexpected format styles value") {
            //                                 "default" => { paragraph_node.block(Block::default()); }
            //                                 &_ => { debug!("Unknown style Value") }
            //                             }
            //                         },
            //                         // "scroll" => {},
            //                         // "wrap" => {},
            //                         &_ => { debug!("Unknown style attr") }
            //                     }
            //                 }
            //             },
            //             _ => { panic!("Unknown Style Format") }
            //         }
            //     }
            //     None => {
            //         El::Paragraph(paragraph_node)
            //     }
            // }
        },
        "Spans" => { 
            match children.len() > 0 {
                true => { 
                    let span_list: Vec<Span> = children.into_iter().map(|child| {
                        match child {
                            El::Span(s) => { s },
                            _ => { panic!("Not a Text Node!") }
                        }
                    }).collect();
                    El::Spans(Spans::from(span_list))
                },
                false => {
                    let text = match el.text {
                        Some(txt) => txt,
                        None => String::from("") 
                    };
                    El::Spans(Spans::from(text))
                }
            }
         
        },
        "Span" => { 
            let text = match el.text {
                Some(txt) => txt,
                None => String::from("") 
            };
            El::Span(Span::from(text))
        },
        &_ => { panic!("Unknown DOM Token") }
    };

    this 
}
