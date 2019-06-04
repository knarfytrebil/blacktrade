extern crate minidom; 
extern crate tui;

use minidom::Element;
use std::collections::HashMap;
use std::fs;
use tui::layout::{Direction, Layout, Constraint};

enum BasicElement {
    ConstraintType(Constraint),
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

fn get_constrant(element: &Element) -> BasicElement {
    BasicElement::ConstraintType(Constraint::Length(1))
}

fn get_layout(element: &Element) -> BasicElement {
    BasicElement::LayoutType(Layout::default())
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
