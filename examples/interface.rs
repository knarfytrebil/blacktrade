extern crate log;
extern crate stderrlog;
extern crate termion; 
extern crate tui;

use std::io;
use std::io::{Write};
use std::thread;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Widget};
use tui::layout::{Direction, Group, Rect, Size};

struct App {
    size: Rect,
}

impl App {
    fn new() -> App {
        App {
            size: Rect::default(),
        }
    }
}

enum Event {
    Input(event::Key),
}

fn main() {
    stderrlog::new().verbosity(4).init().unwrap();

    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    // Input
    thread::spawn(move || {
        let mut input_cmd = String::new();
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    // App
    let mut app = App::new();

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    draw(&mut terminal, &app);

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input {
                event::Key::Char('q') => {
                    // quit
                    break;
                },
                event::Key::Char(':') => {
                    refresh(&mut terminal, &app);
                    println!("change!");
                    // into command mode
                },
                event::Key::Char(';') => {
                    // out of command mode
                }
                _ => {}
            },
        }
        draw(&mut terminal, &app);
    }
    terminal.show_cursor().unwrap();
}

fn refresh(t: &mut Terminal<MouseBackend>, app: &App) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[
            Size::Min(1),
            Size::Fixed(2)
        ])
        .render(t, &app.size, |t, chunks| {
            Block::default()
                .title("Logs")
                .borders(Borders::NONE)
                .render(t, &chunks[0]);

            Block::default()
                .title("Input Command")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::Cyan))
                .render(t, &chunks[1]);
        });
    t.draw().unwrap();
}

fn draw(t: &mut Terminal<MouseBackend>, app: &App) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[
            Size::Min(1),
            Size::Fixed(2)
        ])
        .render(t, &app.size, |t, chunks| {
            Block::default()
                .title("Logs")
                .borders(Borders::NONE)
                .render(t, &chunks[0]);

            Block::default()
                .title("Command")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::Cyan))
                .render(t, &chunks[1]);
        });
    t.draw().unwrap();
}
