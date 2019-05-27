extern crate minidom; 

use std::fs;
use minidom:Element;

let dom_data = fs::read_to_string("./examples/components/articles.html")
    .expect("Error reading file");

fn main() {
    let root: Element = dom_data.parse().unwrap();
}
