use store::app::mode::AppMode;
use store::ui::TopTabs;
use tui::layout::Rect;

#[derive(Clone, Debug)]
pub struct AppState {
    pub mode: AppMode,
    pub size: Rect,
    pub tabs: TopTabs,
    pub command: String,
    pub console_txt: String,
    pub exiting: bool,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            mode: AppMode::get_mode("normal"),
            size: Rect::default(),
            tabs: TopTabs {
                titles: vec![String::from("Console")],
                selection: 0,
            },
            command: String::from(""),
            console_txt: String::from(""),
            exiting: false,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState::new()
    }
}