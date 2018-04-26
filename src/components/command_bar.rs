pub mod command_bar {

    use tui::Terminal;
    use tui::backend::MouseBackend;
    use tui::layout::Rect;
    use tui::widgets::Widget;
    use tui::widgets::Paragraph;
    use store::loops::App;

    pub fn render(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
        Paragraph::default()
            .text("")
            .render(t, area);
    }

}

