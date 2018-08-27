pub mod instance {

    use structs::app::AppState;
    use tui::backend::MouseBackend;
    use tui::layout::Rect;
    use tui::widgets::Block;
    use tui::widgets::Paragraph;
    use tui::widgets::Widget;
    use tui::Terminal;

    pub fn render(t: &mut Terminal<MouseBackend>, app: &AppState, area: &Rect) {
        let scroll = match (app.console_txt.lines().count() as u16).checked_sub(area.height.clone()) {
            None => { 0 as u16 } 
            Some(x) if x > 0 as u16 => { x } 
            Some(_) => { 0 }
        };
        Paragraph::default()
            .block(Block::default())
            .wrap(true)
            .text(&app.console_txt)
            .scroll(scroll)
            .render(t, area);
    }

}
