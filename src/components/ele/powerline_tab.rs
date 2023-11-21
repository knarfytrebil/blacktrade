use unicode_width::UnicodeWidthStr;

use components::ele::powerline_symbol as PowerlineSym;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, Widget};

/// A widget to display available tabs in a multiple panels context.
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, Tabs};
/// # use tui::style::{Style, Color};
/// # use tui::symbols::{DOT};
/// Tabs::default()
///     .block(Block::default().title("Tabs").borders(Borders::ALL))
///     .titles(&["Tab1", "Tab2", "Tab3", "Tab4"])
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().fg(Color::Yellow))
///     .divider(DOT);
/// ```
#[derive(Debug, Clone)]
pub struct Tabs<'a>
{
    /// A block to wrap this widget in if necessary
    block: Option<Block<'a>>,
    /// One title for each tab
    titles: Vec<String>,
    /// The index of the selected tabs
    selected: usize,
    /// The style used to draw the text
    style: Style,
    /// The style used to display the selected item
    highlight_style: Style,
    /// Tab divider
    divider: &'a str,
    /// Tab inactive divider
    divider_inactive: &'a str,
    /// divider style
    divider_style: Style,
}

impl<'a> Default for Tabs<'a>
{
    fn default() -> Tabs<'a> {
        Tabs {
            block: None,
            titles: vec!(String::from("")),
            selected: 0,
            style: Default::default(),
            highlight_style: Default::default(),
            divider: PowerlineSym::RIGHT_ARROW,
            divider_inactive: PowerlineSym::RIGHT_ARROW_LINE,
            divider_style: Default::default(),
        }
    }
}

#[allow(dead_code)]
impl<'a> Tabs<'a>
{
    pub fn block(mut self, block: Block<'a>) -> Tabs<'a> {
        self.block = Some(block);
        self
    }

    pub fn titles(mut self, titles: Vec<String>) -> Tabs<'a> {
        self.titles = titles;
        self
    }

    pub fn select(mut self, selected: usize) -> Tabs<'a> {
        self.selected = selected;
        self
    }

    pub fn style(mut self, style: Style) -> Tabs<'a> {
        self.style = style;
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Tabs<'a> {
        self.highlight_style = style;
        self
    }

    pub fn divider(mut self, divider: &'a str) -> Tabs<'a> {
        self.divider = divider;
        self
    }

    pub fn divider_style(mut self, style: Style) -> Tabs<'a> {
        self.divider_style = style;
        self
    }
}

impl<'a> Widget for Tabs<'a>
{
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let title_padding: u16 = 2;
        buf.set_style(area, self.style);
        let tabs_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if tabs_area.height < 1 {
            return;
        }

        let mut x = tabs_area.left();
        let titles_length = self.titles.len();
        let divider_width = self.divider.width() as u16;
        for (title, style, _last_title, is_selected) in
            self.titles.iter().enumerate().map(|(i, t)| {
                /*
                 * lt: last title      (Boolean)
                 * t:  title           (&String)
                 * i:  index           (int)
                 */
                let lt = i + 1 == titles_length;
                match i == self.selected {
                    true => (t, self.highlight_style, lt, true),
                    false => (t, self.style, lt, false),
                }
            })
        {
            if x >= tabs_area.right() {
                break;
            } else {
                buf.set_string(x, tabs_area.top(), &add_padding(title.as_ref()), style);
                x += title.width() as u16 + title_padding;
                let (divider, divider_style) = match is_selected {
                    true => (self.divider, self.style),
                    false => (self.divider_inactive, self.divider_style),
                };
                if x >= tabs_area.right() {
                    break;
                } else {
                    buf.set_string(x, tabs_area.top(), divider, divider_style);
                    x += divider_width;
                }
            }
            x += 1;
        }
    }
}

fn add_padding(txt: &str) -> String {
    format!(" {} ", txt)
}
