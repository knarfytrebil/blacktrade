pub struct TopTabs<'a> {    
    pub titles: Vec<&'a str>,   
    pub selection: usize,   
}   
    
impl<'a> TopTabs<'a> {  
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
