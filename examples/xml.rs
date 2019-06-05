extern crate minidom; 
extern crate tui;

use minidom::Element;
use std::collections::HashMap;
use std::str::FromStr;
use std::fs;
use tui::layout::{Direction, Layout, Constraint};

enum BasicElement {
    ConstraintType(Constraint),
    DirectionType(Direction),
    LayoutType(Layout),
}

type Callback = fn(&Element) -> BasicElement;

#[derive(Clone)]
struct ElementHandler {
    creator_functions: HashMap<String, Callback>,
}


impl ElementHandler {
    fn new() -> ElementHandler {
        ElementHandler {
            creator_functions: HashMap::new() 
        }
    }

    fn add(&mut self, elementName: String, func: Callback) {
        self.creator_functions.insert(elementName, func);
    }
}

impl FromStr for Direction {
    fn from_str(ele_str: &str) -> Self {
        match ele_str {
            "Vertical" => { Direction::Vertical }
            "Horizontal" => { Direction::Horizontal }
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
    let layout = match element.attr("type") {
        None | Some("default") => { Layout::default() }
        _ => { Layout::default() }
    };
    let d = Direction::From("Vertical");
    for attr in element.attrs() {
    }
    BasicElement::LayoutType(layout)
}

fn tweak_layout(layout: Layout, key: &str, value: &str) {
}


////////////////////////////////////////////////////////////////////////////////////
// Extraction of XML Tree
////////////////////////////////////////////////////////////////////////////////////
fn extract(root: &Element) {
    let mut parser: ElementHandler = ElementHandler::new();    
    parser.add(String::from("Constraint"), get_constrant);
    parser.add(String::from("Layout"), get_layout);
    parseElement(root, parser);
}

// Create Element
fn parseElement(element: &Element, parser: ElementHandler) {
    match is_basic(element) {
        true => { createBasicElement(element, parser); }
        false => { createCustomElement(element, parser); }
    }
}

// Create Basic Element
fn createBasicElement(element: &Element, parser: ElementHandler) {
    println!("Basic element ({:?})", element.name());
    parser.creator_functions[element.name()](element);
    for attr in element.attrs() {
        println!("======= attribute  =======");
        println!("{:#?}", attr);
    }
    if !is_childless(element) {
        println!("Child ({:?})", element.children().count());
        for child in element.children() {
            parseElement(child, parser.clone());
        }
    }
}

// Create Custom Element
fn createCustomElement(element: &Element, parser: ElementHandler) {
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
