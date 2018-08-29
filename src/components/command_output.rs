pub mod instance {

    use structs::app::AppState;
    use tui::backend::MouseBackend;
    use tui::layout::Rect;
    use tui::widgets::Block;
    use tui::widgets::Paragraph;
    use tui::widgets::Widget;
    use tui::Terminal;

    fn get_scroll(line_count: u16, area_height: u16) -> u16 {
        match (line_count).checked_sub(area_height) {
            None | Some(_) => 0 as u16,
            Some(x) if x > 0 as u16 => x,
        }
    }

    fn get_buffer(area_height: u16, txt: String) -> String {
        let mut lines: Vec<usize> = txt.lines().map(|line| line.len()).collect();
        let line_count = lines.len();
        let drained: Vec<usize> = match (line_count as u16).checked_sub(area_height) {
            Some(x) if x >= 0 as u16 => lines.drain(x as usize..line_count).collect(),
            None | Some(_) => lines,
        };
        let drained_bytes: usize = drained.iter().sum();
        match txt.len() {
            l if l <= drained_bytes => txt,
            l if l > drained_bytes => txt[l - drained_bytes - drained.len()..l].to_string(),
            _ => txt,
        }
    }

    pub fn render(t: &mut Terminal<MouseBackend>, app: &AppState, area: Rect) {
        Paragraph::default()
            .block(Block::default())
            .wrap(true)
            .text(&get_buffer(area.height, app.console_txt.clone()))
            .scroll(get_scroll(
                app.console_txt.lines().count() as u16,
                area.height,
            )).render(t, &area);
    }

}
