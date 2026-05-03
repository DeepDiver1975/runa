// LinkList UI component (placeholder for iced integration)
pub struct LinkList {
    pub links: Vec<crate::terminal::Link>,
}

impl LinkList {
    pub fn new() -> Self {
        Self { links: Vec::new() }
    }
    
    pub fn add_link(&mut self, url: String) {
        self.links.push(crate::terminal::Link { range: (0, url.len()), url });
    }
}
