pub mod instance {

    use structs::app::AppState;
    use tui::backend::MouseBackend;
    use tui::layout::Rect;
    use tui::widgets::Block;
    use tui::widgets::Paragraph;
    use tui::widgets::Widget;
    use tui::Terminal;

    pub fn render(t: &mut Terminal<MouseBackend>, _app: &AppState, area: &Rect) {
        let buffer = _app.console_txt.clone();
        Paragraph::default()
            .block(Block::default())
            .wrap(true)
            .text(&buffer)
            .render(t, area);
    }

}
