pub mod instance {

    use store::app::AppState;
    use tui::backend::MouseBackend;
    use tui::layout::Rect;
    use tui::widgets::Paragraph;
    use tui::widgets::Widget;
    use tui::Terminal;

    pub fn render(t: &mut Terminal<MouseBackend>, _app: &AppState, area: &Rect) {
        Paragraph::default().text(&_app.mode.symbol).render(t, area);
    }

}
