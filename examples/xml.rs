extern crate minidom; 

use std::fs;
use minidom::Element;

fn main() {
    let dom_data = fs::read_to_string("./examples/components/articles.xml")
        .expect("Error reading file");
    let root: Element = dom_data.parse().unwrap();
    println!("{:#?}", root);
}
