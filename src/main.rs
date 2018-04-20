extern crate log;
extern crate stderrlog;
extern crate termion; 
extern crate tui;

mod store;

use std::io;
use std::io::{Write};
use std::thread;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Widget, Paragraph, Tabs};
use tui::layout::{Direction, Group, Rect, Size};

use store::*;

struct App<'a> {
    size: Rect,
    tabs: TopTabs<'a>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            size: Rect::default(),
            tabs: TopTabs {
                titles: vec!["Poloniex", "Logs"],
                selection: 0,
            }
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
    renderApp(&mut terminal, &app);
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
                    break;
                }
               _ => {}
            },
        }
        renderApp(&mut terminal, &app);
    }
    terminal.show_cursor().unwrap();
}

fn renderApp(t: &mut Terminal<MouseBackend>, app: &App) -> Result<(), io::Error> {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(3), Size::Min(1), Size::Fixed(1), Size::Fixed(1)])
        .render(t, &app.size, |t, chunks| {
            Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .titles(&app.tabs.titles)
                .style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(app.tabs.selection)
                .render(t, &chunks[0]);
            match app.tabs.selection {
                0 => { renderText(t, app, &chunks[1]) }
                1 => { }
                _ => { }
            }
            renderStatusBar(t, app, &chunks[2]);
            renderCommandBar(t, app, &chunks[3]);
        });
    try!(t.draw());
    Ok(())
}

fn renderText(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
     Paragraph::default()
        .block(Block::default().title("Text"))
        .wrap(true)
        .text("text")
        .render(t, area);
}

fn renderStatusBar(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Paragraph::default()
        .text("Paragraph 1")
        .render(t, area);
}

fn renderCommandBar(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Paragraph::default()
        .text("Paragraph 2")
        .render(t, area);
}
