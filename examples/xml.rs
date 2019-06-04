extern crate minidom; 
extern crate tui;

use minidom::Element;
use std::collections::HashMap;
use std::fs;
use tui::layout::{Direction, Layout, Constraint};

enum BasicElement {
}

type Callback = fn(&Element) -> BasicElement;

struct ElementHandler {
    creator_function: HashMap<String, Callback>,
}

impl ElementHandler {
    fn new() -> ElementHandler {
        ElementHandler {
            creator_function: HashMap::new() 
        }
    }

    fn add(&mut self, elementName: String, func: Callback) {
        self.creator_function.insert(elementName, func);
    }
}

fn get_constrant(element: &Element) -> BasicElement  {
}

////////////////////////////////////////////////////////////////////////////////////
// Extraction of XML Tree
////////////////////////////////////////////////////////////////////////////////////
fn extract(root: &Element) {
    let parser: ElementHandler = ElementHandler::new();    
    parser.add(String::from("Constraint"), get_constrant);
    parseElement(root);
}

// Create Element
fn parseElement(element: &Element) {
    match is_basic(element) {
        true => { createBasicElement(element); }
        false => { createCustomElement(element); }
    }
}

// Create Basic Element
fn createBasicElement(element: &Element) {
    println!("Basic element ({:?})", element.name());
    for attr in element.attrs() {
        println!("======= attribute  =======");
        println!("{:#?}", attr);
    }
    if !is_childless(element) {
        println!("Child ({:?})", element.children().count());
        for child in element.children() {
            parseElement(child);
        }
    }
}

// Create Custom Element
fn createCustomElement(element: &Element) {
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
