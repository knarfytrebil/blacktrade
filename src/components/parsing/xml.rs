use std::convert::TryInto;
use std::str::FromStr;
use treexml::{Document, Element};
use serde_json::{Value, Map};
use handlebars::Handlebars;
use ratatui::style::{Color, Style};
use ratatui::layout::Alignment;
use structs::ui::TopTabs;

pub fn parse_xml(xml: String) -> Element {
    let doc = Document::parse(xml.as_bytes()).expect("XML Parse Error");
    doc.root.expect("XML Parse Error")
}

pub fn parse<'a>(template_name: &'a str, v: &Value, reg: &mut Handlebars<'_>) -> Element {
    let filled_template = reg
        .render(template_name, &v)
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
    parse_res
}

pub fn parse_text_attr<'a>(el: Element, attr_name: &'a str) -> Option<String> {
    match el.attributes.contains_key(attr_name) {
        true => Some(String::from(&el.attributes[attr_name])),
        false => None
    }
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