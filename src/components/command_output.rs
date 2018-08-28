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
            None | Some(_) => { 0 as u16 } 
            Some(x) if x > 0 as u16 => { x } 
        }
    }

    fn get_buffer(area_height: u16, txt: String) -> String {
        debug!("txt_len, incoming:{:?}", txt.len());
        let mut lines: Vec<usize> = txt.lines().map(|line| { line.len() }).collect();
        debug!("lines:{:?}", lines);
        let line_count = lines.len();
        debug!("line_count:{:?}", line_count);
        let drained: Vec<usize> = match (line_count as u16).checked_sub(area_height) {
            None | Some(_) => { lines }
            Some(x) if x >= 0 as u16 => { lines.drain(line_count - (area_height as usize)..line_count).collect() }
        };
        debug!("drained:{:?}", drained);
        let drained_bytes: usize = drained.iter().fold(0, |sum, i| sum + i + 2);
        debug!("drained_bytes:{:?}", drained_bytes);
        debug!("txt_len:{:?}", txt.len());
        match txt.len() {
            l if l <= drained_bytes => { txt }
            l if l > drained_bytes => { txt[l - drained_bytes..l].to_string() }
            _ => { txt }
        }
    }

    pub fn render(t: &mut Terminal<MouseBackend>, app: &AppState, area: &Rect) {
        Paragraph::default()
            .block(Block::default())
            .wrap(true)
            .text(&get_buffer(area.height.clone(), app.console_txt.clone()))
            .scroll(get_scroll(app.console_txt.lines().count() as u16, area.height.clone()))
            .render(t, area);
    }

}
