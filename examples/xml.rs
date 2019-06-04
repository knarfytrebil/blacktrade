extern crate minidom; 
use std::fs;
use minidom::Element;

fn main() {
    let dom_data = fs::read_to_string("./examples/components/index.xml")
        .expect("Error reading file");
    let root: Element = dom_data.parse().unwrap();
    extract(&root);
}

// Extraction of XML Tree
fn extract(root: &Element) {
    createElement(root);
    // Recursive
    for child in root.children() {
        extract(child);
    }
}

// Create Element
fn createElement(element: &Element) {
    // println!("{:#?}", element);
    println!("Create Element {:?}", element.name());
    println!("Is Original: {:#?}", is_original(element));

    for attr in element.attrs() {
        println!("======= attribute  =======");
        println!("{:#?}", attr);
    }
}

// Utility Functions

// Element has not attribute and not Child element
// AND DOES NOT belong to standard widget list
fn is_original(element: &Element) -> bool {
    return is_attrless(element) && is_childless(element);
}

// Element has no Child Element
fn is_childless(element: &Element) -> bool {
    return element.children().count() == 0;
}

// Element has no attribute 
fn is_attrless(element: &Element) -> bool {
    return element.attrs().count() == 0;
}
