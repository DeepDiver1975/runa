use std::path::PathBuf;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct FileTree { pub root: PathBuf, pub expanded: HashSet<PathBuf> }

impl FileTree {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self { 
        Self { root: root.into(), expanded: HashSet::new() } 
    }
    
    pub fn toggle_expand(&mut self, p: &PathBuf) {
        if self.expanded.contains(p) { 
            self.expanded.remove(p); 
        } else { 
            self.expanded.insert(p.clone()); 
        }
    }
    
    pub fn is_expanded(&self, p: &PathBuf) -> bool { 
        self.expanded.contains(p) 
    }
}
