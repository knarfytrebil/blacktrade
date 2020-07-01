use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    let _res = file.read_to_string(&mut contents);
    contents
}
