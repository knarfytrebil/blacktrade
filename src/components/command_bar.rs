pub mod instance {

    use std::io;
    use structs::app::AppState;
    use tui::backend::TermionBackend;
    use tui::layout::Rect;
    use tui::widgets::Paragraph;
    use tui::widgets::Widget;
    use tui::Terminal;

    pub fn render(t: &mut Terminal<TermionBackend<io::Write>>, _app: &AppState, area: Rect) {
        Paragraph::default().text(&_app.command).render(t, &area);
    }

}
