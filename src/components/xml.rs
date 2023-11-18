use std::convert::TryInto;
use std::str::FromStr;
use handlebars::{Handlebars,handlebars_helper};
use serde_json::{Value, Map};
use treexml::{Document, Element};
use ratatui::layout::{Constraint, Direction, Layout, Alignment};
use ratatui::style::{Color, Style};
use ratatui::text::{Span, Line};
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;
use components::ele::powerline_tab::Tabs;
use structs::ui::TopTabs;

pub enum El {
    Paragraph(Paragraph<'static>),
    Line(Line<'static>),
    Span(Span<'static>),
    Tabs(Tabs<'static>),
    Layout(Layout),
    Constraint(Constraint),
}

// Handlebar Helpers
handlebars_helper!(stringify: |v: Json| {
    v.to_string()
});

// Handlebar escape fn
pub fn escape_nothing(data: &str) -> String {
    String::from(data)
}

pub fn parse_xml(xml: String) -> Element {
    let doc = Document::parse(xml.as_bytes()).expect("XML Parse Error");
    doc.root.expect("XML Parse Error")
}

pub fn parse(template: String, v: Option<&Value>) -> Element {
    let filled_template = match v {
        Some(v) => {
            let mut reg = Handlebars::new();

            reg.register_helper("stringify", Box::new(stringify));
            reg.register_escape_fn(escape_nothing);

            reg
                .render_template(&template, &v)
                .expect("Template Parse Error")

        },
        None => {
            template
        }
    };
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
    parse_res
}

pub fn parse_tabs(el: Element) -> Option<TopTabs> {
    match el.attributes.contains_key("tabs") {
        true => match serde_json::from_str(&el.attributes["tabs"]) {
            Ok(tabs) => Some(tabs),
            Err(_) => {
                debug!("Attribute Parse Error: {:?}", &el.attributes["tabs"]);
                None
            }
        },
        false => {
            debug!("Unable to find tabs attribute");
            None
        },
    }
}

pub fn extract_text(el: Element) -> String {
    match el.text {
        Some(txt) => txt,
        // allows empty Line or Span
        None => String::from(""),
    }
}

pub fn apply_color<'a>(style: Style, v_styles: &Value, color_attr: &'a str) -> Style {
    match v_styles.get(color_attr).and_then(|value| value.as_str()) {
        Some(color_str) => {
            let color = Color::from_str(color_str).unwrap();
            match color_attr {
                "fg" => style.fg(color),
                "bg" => style.bg(color),
                _ => style
            }
        }
        None => style
    }
}

pub fn parse_styles<'a>(el: Element, attr_name: &'a str) -> Style {
    let styles_json: Option<Value> = parse_attr(el.clone(), attr_name);
    let mut style = Style::default();

    if let Some(ref v_styles) = styles_json {
        style = apply_color(style, &v_styles, "fg");
        style = apply_color(style, &v_styles, "bg");
    }
    style
}

pub fn alignment_from_text<'a>(txt_alignment: &'a str) -> Alignment {
    // Default to Left
    match txt_alignment {
        "Center" => Alignment::Center,
        "Right" => Alignment::Right,
        "Left" | _ => Alignment::Left,
    }
}

pub fn get_u16_value<'a>(obj: &Map<String, Value>, key: &'a str) -> u16 {
    let err_msg = format!("{} value error", key);
    obj.get(key).unwrap().as_u64().expect(&err_msg).try_into().expect(&err_msg)
}

pub fn get_ratio_value<'a>(obj: &Map<String, Value>, key: &'a str) -> Vec<u32> {
    let obj_value = obj
        .get(key).unwrap().as_str().expect("ratio value error");
    let ratio: Vec<u32> = obj_value
        .split(":")
        .into_iter()
        .map(|v| {
            v.to_string().parse()
            .expect("ratio value parse error")
        })
        .collect();
    match ratio.len() {
        2 => ratio,
        _ => panic!("Ratio must be in the form of '1:2'"),
    }
}

pub fn create_element(el: Element) -> El {
    // Children Section
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

    let style = parse_styles(el.clone(), "styles");

    // Check Common Attributes
    // All Elements has Styles, so all styles needed to be parsed here.
    let this = match el.name.as_str() {

        // A widget to display some text.
        "Paragraph" => {
            // Attribute Unqiue to "Paragraph"
            // Attribute will be tralsated into Methods
            let wrap_json: Option<Value> = parse_attr(el.clone(), "wrap");
            let alignment_json: Option<Value> = parse_attr(el.clone(), "alignment");

            // Children
            let el_list: Vec<Line> = match !children.is_empty() {
                true => children
                    .into_iter()
                    .map(|child| match child {
                        El::Line(s) => s,
                        _ => panic!("Not a Text Node!"),
                    })
                    .collect(),
                false => vec![],
            };
            let mut paragraph_el = Paragraph::new(el_list);

            // Handle Wrap
            if let Some(v_wrap) = wrap_json {
                if let Some(trim) = v_wrap.get("trim").and_then(|value| value.as_bool()) {
                    paragraph_el = paragraph_el.wrap(Wrap{trim:trim})
                }
            }

            // Handle Alignment
            if let Some(v_alignment) = alignment_json {
                if let Some(alignment_str) =
                    v_alignment.get("position").and_then(|value| value.as_str())
                {
                    paragraph_el = paragraph_el.alignment(alignment_from_text(alignment_str))
                }
            }

            paragraph_el = paragraph_el.style(style);
            El::Paragraph(paragraph_el)
        }
        "Line" => match !children.is_empty() {
            true => {
                let span_list: Vec<Span> = children
                    .into_iter()
                    .map(|child| match child {
                        El::Span(s) => s,
                        _ => panic!("Not a Text Node!"),
                    })
                    .collect();
                El::Line(Line::from(span_list))
            }
            false => El::Line(Line::from(extract_text(el))),
        },
        "Span" => { 
            let span_el = Span::styled(extract_text(el), style);
            El::Span(span_el)
        },
        "Tabs" => {
            let mut tabs_el = Tabs::default();
            let tabs: TopTabs = parse_tabs(el.clone()).unwrap();
            let hightlight_style = parse_styles(el.clone(), "highlight_styles");
            let divider_style = parse_styles(el.clone(), "divider_styles");
            tabs_el = tabs_el
                .titles(tabs.titles)
                .highlight_style(hightlight_style)
                .divider_style(divider_style)
                .select(tabs.selection);
            El::Tabs(tabs_el)
        },
        "Layout" => {
            let direction_json: Option<Value> = parse_attr(el.clone(), "direction");
            let mut layout_el = Layout::default();
            if let Some(v_direction) = direction_json {
                match v_direction.as_str() {
                    Some("vertical") => layout_el = layout_el.direction(Direction::Vertical),
                    Some("horizontal") => layout_el = layout_el.direction(Direction::Horizontal),
                    _ => panic!("Unknown Direction"),
                }
            }

            // Children
            let el_list: Vec<Constraint> = match !children.is_empty() {
                true => children
                    .into_iter()
                    .map(|child| match child {
                        El::Constraint(s) => s,
                        _ => panic!("Not a Constraint Node!"),
                    })
                    .collect(),
                false => vec![],
            };

            layout_el = layout_el.constraints(el_list);
 
            El::Layout(layout_el)
        },
        "Constraint" => {
            if let Some(value) = parse_attr(el.clone(), "type") {
                if value.is_object() {
                    let obj = value.as_object()
                        .expect("object values are wrong");
                    let key = obj.keys().last().unwrap().as_str();
                    let constraint_el = match key {
                        "length" => Constraint::Length(get_u16_value(obj, key)),
                        "min" => Constraint::Min(get_u16_value(obj, key)),
                        "max" => Constraint::Max(get_u16_value(obj, key)),
                        "percentage" => Constraint::Percentage(get_u16_value(obj, key)),
                        "ratio" => {
                            let rv= get_ratio_value(obj, key);
                            Constraint::Ratio(rv[0],rv[1])
                        },
                        _ => panic!("Wrong type for constraint")
                    };
                    El::Constraint(constraint_el)
                } else {
                    panic!("constraint type value must be a json object");
                }
            } else {
                panic!("constraint type value must be a json object");
            }
        },
        &_ => {
            panic!("Unknown DOM Token")
        } 
    };

    this
}
