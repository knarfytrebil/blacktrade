use handlebars::{handlebars_helper, Handlebars};
use serde::de::value;
use serde_json::Value;
use termion::scroll;
use treexml::{Document, Element};
use tui::layout::Alignment;
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;
use tui::widgets::{Block, Wrap};

pub enum El {
    Paragraph(Paragraph<'static>),
    Spans(Spans<'static>),
    Span(Span<'static>),
}

pub fn parse_xml(xml: String) -> Element {
    let doc = Document::parse(xml.as_bytes()).unwrap();
    doc.root.unwrap()
}

// fn generate_line_buffer(height: u16) -> Box<dyn Fn(Vec<Value>) -> Vec<Value>> {
//     handlebars_helper!(line_buffer: |v: Vec| {
//         let buffer_start = match height as usize <= lines.len() {
//             false => 0,
//             true => lines.len() - height as usize
//         };
//         (&lines[buffer_start..]).to_vec()
//
//     })
// }

// fn inner_buffer(area_height: u16, lines: Vec<Value>) -> Vec<Value> {
//     let buffer_start = match area_height as usize <= lines.len() {
//         false => 0,
//         true => lines.len() - area_height as usize
//     };
//     (&lines[buffer_start..]).to_vec()
// }

pub fn parse(template: String, v: &Value) -> Element {
    let reg = Handlebars::new();
    // reg.register_helper("line_buffer", line_buffer(v["metrics"]["height"].as_u16()));

    let filled_template = reg
        .render_template(&template, &v)
        .expect("Template Parse Error");
    parse_xml(filled_template)
}

pub fn parse_attr<'a>(el: Element, attr_name: &'a str) -> Option<Value> {
    let parse_res = match el.attributes.contains_key(attr_name) {
        true => match serde_json::from_str(&el.attributes[attr_name]) {
            Ok(value) => Some(value),
            Err(err) => {
                debug!("Attribute Parse Error: {:?}", err);
                None
            }
        },
        false => None,
    };
    debug!("{}: {:?}", attr_name, parse_res);
    parse_res
}

pub fn extract_text(el: Element) -> String {
    match el.text {
        Some(txt) => txt,
        // allows empty Spans or Span
        None => String::from(""),
    }
}

pub fn alignment_from_text<'a>(txt_alignment: &'a str) -> Alignment {
    // Default to Left
    match txt_alignment {
        "Center" => Alignment::Center,
        "Right" => Alignment::Right,
        "Left" | _ => Alignment::Left,
    }
}

pub fn create_element(el: Element) -> El {
    let children: Vec<El> = match !el.children.is_empty() {
        // recursive till there is no more child elements
        true => el
            .children
            .clone()
            .into_iter()
            .map(create_element)
            .collect(),
        false => vec![],
    };

    // Check Common Attributes
    // All Elements has Styles, so all styles needed to be parsed here.
    let styles_json: Option<Value> = parse_attr(el.clone(), "styles");

    let this = match el.name.as_str() {
        // A widget to display some text.
        "Paragraph" => {
            // Attribute Unqiue to "Paragraph"
            // Attribute will be tralsated into Methods
            let wrap_json: Option<Value> = parse_attr(el.clone(), "wrap");
            let scroll_json: Option<Value> = parse_attr(el.clone(), "scroll");
            let alignment_json: Option<Value> = parse_attr(el.clone(), "alignment");

            let el_list: Vec<Spans> = match !children.is_empty() {
                true => children
                    .into_iter()
                    .map(|child| match child {
                        El::Spans(s) => s,
                        _ => panic!("Not a Text Node!"),
                    })
                    .collect(),
                false => vec![],
            };
            let mut paragraph_el = Paragraph::new(el_list);

            if let Some(v_wrap) = wrap_json {
                if let Some(trim) = v_wrap.get("trim").and_then(|value| value.as_bool()) {
                    paragraph_el = paragraph_el.wrap(Wrap { trim: trim })
                }
            }

            if let Some(v_alignment) = alignment_json {
                if let Some(alignment_str) =
                    v_alignment.get("position").and_then(|value| value.as_str())
                {
                    paragraph_el = paragraph_el.alignment(alignment_from_text(alignment_str))
                }
            }

            // match styles {
            //     Some(style) => El::Paragraph( Paragraph::new(el_list).style(style)),
            //     None => El::Paragraph( Paragraph::new(el_list))
            // }

            El::Paragraph(paragraph_el)
        }
        "Spans" => match !children.is_empty() {
            true => {
                let span_list: Vec<Span> = children
                    .into_iter()
                    .map(|child| match child {
                        El::Span(s) => s,
                        _ => panic!("Not a Text Node!"),
                    })
                    .collect();
                El::Spans(Spans::from(span_list))
            }
            false => El::Spans(Spans::from(extract_text(el))),
        },
        "Span" => El::Span(Span::from(extract_text(el))),
        &_ => panic!("Unknown DOM Token"),
    };

    this
}
