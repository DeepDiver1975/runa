// Sidebar UI component (placeholder for iced integration)
pub struct Sidebar {
    pub selected: Option<std::path::PathBuf>,
}

impl Sidebar {
    pub fn new() -> Self {
        Self { selected: None }
    }
}
