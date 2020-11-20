#[macro_use]
extern crate log;
extern crate simplelog;
extern crate tui;
use tui::widgets::{Paragraph, Text};

use simplelog::*;

use std::fs::File;

fn main() {
    // CombinedLogger::init(
    //     vec![
    //         TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
    //         WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
    //     ]
    // ).unwrap();

    // error!("Bright red error");
    // info!("This only appears in the log file");
    // debug!("This level is currently not enabled for any logger");
    let buffer = String::from("foo\r\nbar\n\nbaz\n");
    let text: Vec<Text> = buffer
        .lines()
        .into_iter()
        .map(|line| Text::raw(line))
        .rev()
        .collect();
    // println!("{:?}", text);
}
