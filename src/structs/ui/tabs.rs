use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopTabs {
    pub titles: Vec<String>,
    pub selection: usize,
}

impl TopTabs {
    pub fn next(&mut self) {
        self.selection = (self.selection + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.selection > 0 {
            self.selection -= 1;
        } else {
            self.selection = self.titles.len() - 1;
        }
    }
}
