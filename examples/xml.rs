extern crate minidom; 
extern crate tui;

use std::collections::HashMap;
use std::vec::Vec;
use std::fs;

use tui::layout::{Direction, Layout, Constraint, Rect};
use tui::widgets::{Tabs, Widget, Paragraph, Text};
use tui::backend::{Backend};
use tui::Frame;
use minidom::Element;

type CallbackFn = fn(&Element) -> BasicElement;

enum Props {
    StringRef(&'static str)
}

/* ------------------------------------ 
 * List of Widgets:
 * ------------------------------------
 * BarChart 
 * Map, Line, Points, Rectangle, World 
 * Block
 * Chart
 * Gauge
 * List
 * Reflow                    [No Size]
 * Paragraph                 [No Size]
 * Sparkline
 * Table
 * Tabs
 * ---------------------------------- */ 
enum BasicElement {
    ConstraintT(Constraint),
    LayoutT(Layout),
    TabsT(Tabs<'static, &'static str>),
    ParagraphT(Vec<Text<'static>>)
}

/* ------------------ 
 * Basic Attributes
 * ------------------ 
 * D: Direction
 * M: Margin
 * ------------------ */ 
#[derive(Debug)]
enum BaseAttr {
    D(Direction),
    M(u16),
}

#[derive(Clone)]
struct TuiParser {
    creator_functions: HashMap<&'static str, CallbackFn>,
}

trait ElementHandler {
    fn new(el_names: Vec<&'static str>) -> Self; 

    fn assemble_fn(&mut self, el_names: &[&'static str]) {
        for el_name in el_names {
            self.add(el_name);
        }
    }

    fn add(&mut self, el_name: &'static str) {
        self.push_el_fn(el_name, Self::el_fn(el_name).unwrap());
    }

    // Instance method Sig
    fn push_el_fn(&mut self, el_name: &'static str, func: CallbackFn); 

    // Static method Sig
    fn el_fn(el_name: &str) -> Option<CallbackFn>; 
    fn get_attr(el: &Element, key: &str) -> Option<BaseAttr>; 
}

impl Default for TuiParser {
    fn default() -> TuiParser {
        TuiParser {
            creator_functions: HashMap::new() 
        }
    }
}

impl ElementHandler for TuiParser {
    fn new(el_names: Vec<&'static str>) -> Self {
        let mut eh = Self::default();
        eh.assemble_fn(&el_names);
        eh
    }

    fn push_el_fn(&mut self, el_name: &'static str, func: CallbackFn) {
        self.creator_functions.insert(el_name, func);
    }

    fn el_fn(el_name: &str) -> Option<CallbackFn> {
        match el_name {
            "Constraint" => { Some(get_constrant) }
            "Layout" => { Some(get_layout) }
            "Tabs" => { Some(get_tabs) }
            "Paragraph" => { Some(get_paragraph) }
            _ => { None }
        }
    }

    fn get_attr(el: &Element, key: &str) -> Option<BaseAttr> {
        match key {
            "direction" => {
                match el.attr(key) {
                    Some("Horizontal") => { Some(BaseAttr::D(Direction::Horizontal)) }
                    Some("Vertical") => { Some(BaseAttr::D(Direction::Vertical)) }
                    _ | None => { None }
                }
            }
            "margin" => {
                match el.attr(key) {
                    Some(mgn) => { Some(BaseAttr::M(mgn.to_string().parse::<u16>().unwrap())) }
                    _ | None => { None }
                }
            }
            _ => { None }
        }
    }

    
}

//////////////////////////////
// Basic Element Generation //
//////////////////////////////
fn get_constrant(element: &Element) -> BasicElement {
    let attr = element.attrs().next().unwrap();
    let value: u16 = attr.1.to_string().parse().unwrap();
    let constraint = match attr.0 {
        "Length" => { Constraint::Length(value) } 
        "Max" => { Constraint::Max(value) }
        "Min" |  _ => { Constraint::Min(value) }
    };
    BasicElement::ConstraintT(constraint)
}

fn get_layout(el: &Element) -> BasicElement {
    let mut layout = Layout::default(); 
    for attr in el.attrs() {
        layout = match TuiParser::get_attr(el, attr.0) {
            Some(parsed_attr) => { 
                match parsed_attr {
                    BaseAttr::D(dir) => { layout.direction(dir) }
                    BaseAttr::M(mgn) => { layout.margin(mgn) }
                    _ => { layout.direction(Direction::Horizontal) } 
                }
            }
            None => { 
                println!("Non Native Attribute Found {:#?}", attr.0);
                layout 
            }
        };
    }
    println!("{:#?}", layout);
    BasicElement::LayoutT(layout)
}

fn get_tabs(el: &Element) -> BasicElement {
    let mut tabs = Tabs::default();
    BasicElement::TabsT(tabs)
}

fn get_paragraph(el: &Element) -> BasicElement {
    let v = vec![Text::raw("wtf")];
    BasicElement::ParagraphT(v)
}

/*************************
 * Extraction of XML Tree
 *************************/
fn extract(root: &Element) {
    let parser = TuiParser::new(vec!["Constraint", "Layout", "Tabs", "Paragraph"]);
    parse_element(root, parser);
}

// Create Element
fn parse_element(element: &Element, parser: TuiParser) {
    match is_basic(element) {
        true => { create_basic_element(element, parser); }
        false => { create_custom_element(element, parser); }
    }
}

// Create Basic Element
fn create_basic_element(el: &Element, parser: TuiParser) {
    println!("Basic element ({:?})", el.name());
    let base_el = parser.creator_functions[el.name()](el);
    if !is_childless(el) {
        println!("Child ({:?})", el.children().count());
        for child in el.children() {
            parse_element(child, parser.clone());
        }
    }
}

// Create Custom Element
fn create_custom_element(element: &Element, parser: TuiParser) {
    println!("<====== Custom");
    println!("Custom Element ({:#?})", element.name());
    println!("Custom ======>");
}

// Utility Functions
// Element has not attribute and not Child element
// AND DOES NOT belong to standard widget list
fn is_basic(element: &Element) -> bool {
    return !(is_attrless(element) && is_childless(element));
}

// Element has no Child Element
fn is_childless(element: &Element) -> bool {
    return element.children().count() == 0;
}

// Element has no attribute 
fn is_attrless(element: &Element) -> bool {
    return element.attrs().count() == 0;
}

// Main Function
fn main() {
    let dom_data = fs::read_to_string("./examples/components/index.xml")
        .expect("Error reading file");
    let root: Element = dom_data.parse().unwrap();
    extract(&root);
}
