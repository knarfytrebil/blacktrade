pub mod instance {

    use tui::Terminal;
    use tui::backend::MouseBackend;
    use tui::layout::Rect;
    use tui::widgets::Widget;
    use tui::widgets::Paragraph;
    use store::loops::AppState;

    pub fn render(t: &mut Terminal<MouseBackend>, _app: &AppState, area: &Rect) {
        Paragraph::default()
            .text(&_app.command)
            .render(t, area);
    }

}

