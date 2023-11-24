use serde_json::Value;
use treexml::Element;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::{Span, Line};
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;
use components::ele::powerline_tab::Tabs;
use components::parsing::xml::{
    parse_attr,
    parse_text_attr,
    parse_str_list,
    parse_usize,
    parse_styles,
    extract_text, alignment_from_text, 
    get_ratio_value, get_u16_value
};

#[derive(Clone)]
pub enum El {
    Paragraph(Paragraph<'static>),
    Line(Line<'static>),
    Span(Span<'static>),
    Tabs(Tabs<'static>),
    Layout(Layout, Vec<Option<String>>),
    Constraint(Constraint, Option<String>),
    Component(Option<String>),
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
                    paragraph_el = paragraph_el.wrap(Wrap{trim})
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
            let tabs_titles = parse_str_list(el.clone(), "tab_titles").unwrap();
            let tabs_selection = parse_usize(el.clone(), "tab_selection").unwrap();
            let hightlight_style = parse_styles(el.clone(), "highlight_styles");
            let divider_style = parse_styles(el.clone(), "divider_styles");
            tabs_el = tabs_el
                .titles(tabs_titles)
                .highlight_style(hightlight_style)
                .divider_style(divider_style)
                .select(tabs_selection);
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
                    .clone()
                    .into_iter()
                    .map(|child| match child {
                        El::Constraint(c, _) => c,
                        _ => panic!("Not a Constraint Node!"),
                    })
                    .collect(),
                false => vec![],
            };

            let template_list: Vec<Option<String>> = match !children.clone().is_empty() {
                true => children
                    .into_iter()
                    .map(|child| match child {
                        El::Constraint(_, s) => {
                           s
                        },
                        _ => panic!("Not a Constraint Node!"),
                    })
                    .collect(),
                false => vec![],
            };

            layout_el = layout_el.constraints(el_list);
 
            El::Layout(layout_el, template_list)
        },
        "Constraint" => {
            if let Some(value) = parse_attr(el.clone(), "type") {
                if value.is_object() {
                    let obj = value
                        .as_object()
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
                    // Children
                    match !children.is_empty() {
                        true => {
                            let components: Vec<Option<String>> = children
                            .into_iter()
                            .map(|child| match child {
                                El::Component(c) => c,
                                _ => panic!("Not a Component Node!"),
                            })
                            .collect();
                            El::Constraint(constraint_el, components[0].clone())
                        },
                        false => {
                            El::Constraint(constraint_el, None)
                        },
                    }
                } else {
                    panic!("constraint type value must be a json object");
                }
            } else {
                panic!("constraint type value must be a json object");
            }
        },
        "Component" => {
            let template = parse_text_attr(el.clone(), "template");
            El::Component(template)
        }, 
        _ => panic!("Unknown Element"),
    };

    this
}
