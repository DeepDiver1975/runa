use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum StepStatus { Pending, Running, Done, Error }

#[derive(Debug, Clone)]
pub struct Step {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub code: String,
    pub status: StepStatus,
}

#[derive(Debug)]
pub struct AppState {
    pub steps: Vec<Step>,
    pub active_step: usize,
}

impl AppState {
    pub fn new() -> Self { Self { steps: Vec::new(), active_step: 0 } }
    
    pub fn next_step(&mut self) {
        if self.steps.is_empty() { return }
        self.active_step = (self.active_step + 1) % self.steps.len();
    }
    
    pub fn prev_step(&mut self) {
        if self.steps.is_empty() { return }
        if self.active_step == 0 { self.active_step = self.steps.len()-1 } else { self.active_step -= 1 }
    }
}

impl Step {
    pub fn dummy(title: &str) -> Self {
        Self { id: Uuid::new_v4(), title: title.to_string(), description: "".into(), code: "".into(), status: StepStatus::Pending }
    }
}
