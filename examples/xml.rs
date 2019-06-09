extern crate minidom; 
extern crate tui;

use minidom::Element;
use std::collections::HashMap;
use std::vec::Vec;

use std::fs;
use tui::layout::{Direction, Layout, Constraint};

enum BasicElement {
    ConstraintType(Constraint),
    LayoutType(Layout),
}

#[derive(Debug)]
enum BaseAttr {
    D(Direction),
}

type Callback = fn(&Element) -> BasicElement;

#[derive(Clone)]
struct TuiParser {
    creator_functions: HashMap<&'static str, Callback>,
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
    fn push_el_fn(&mut self, el_name: &'static str, func: Callback); 

    // Static method Sig
    fn el_fn(el_name: &str) -> Option<Callback>; 
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

    fn push_el_fn(&mut self, el_name: &'static str, func: Callback) {
        self.creator_functions.insert(el_name, func);
    }

    fn el_fn(el_name: &str) -> Option<Callback> {
        match el_name {
            "Constraint" => { Some(get_constrant) }
            "Layout" => { Some(get_layout) }
            _ => { None }
        }
    }

    fn get_attr(el: &Element, key: &str) -> Option<BaseAttr> {
        match el.attr(key) {
            Some("Horizontal") => { Some(BaseAttr::D(Direction::Horizontal)) }
            Some("Vertical") => { Some(BaseAttr::D(Direction::Vertical)) }
            _ | None => { None }
        }
    }
}

fn get_constrant(element: &Element) -> BasicElement {
    let attr = element.attrs().next().unwrap();
    let value: u16 = attr.1.to_string().parse().unwrap();
    let constraint = match attr.0 {
        "Length" => { Constraint::Length(value) } 
        "Max" => { Constraint::Max(value) }
        "Min" |  _ => { Constraint::Min(value) }
    };
    BasicElement::ConstraintType(constraint)
}

fn get_layout(element: &Element) -> BasicElement {
    let mut layout = Layout::default(); 
    let BaseAttr::D(dir) = TuiParser::get_attr(element, "direction").unwrap();
    layout = layout.direction(dir);
    println!("{:#?}", layout);
    BasicElement::LayoutType(layout)
}

////////////////////////////
// Extraction of XML Tree //
////////////////////////////
fn extract(root: &Element) {
    let parser = TuiParser::new(vec!["Constraint", "Layout"]);    
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
fn create_basic_element(element: &Element, parser: TuiParser) {
    println!("Basic element ({:?})", element.name());
    parser.creator_functions[element.name()](element);

    if !is_childless(element) {
        println!("Child ({:?})", element.children().count());
        for child in element.children() {
            parse_element(child, parser.clone());
        }
    }
}

// Create Custom Element
fn create_custom_element(element: &Element, parser: TuiParser) {
    println!("======= CUSTOM  =======");
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
