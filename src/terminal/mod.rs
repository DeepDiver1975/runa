use async_trait::async_trait;
use uuid::Uuid;
use std::collections::VecDeque;

pub type SessionId = Uuid;

#[async_trait]
pub trait TerminalBackend: Send + Sync {
    async fn spawn(&mut self, cmd: &str) -> anyhow::Result<SessionId>;
    async fn write(&mut self, id: SessionId, input: &str) -> anyhow::Result<()>;
    async fn kill(&mut self, id: SessionId) -> anyhow::Result<()>;
}

pub struct MockTerminal {
    outputs: std::collections::HashMap<SessionId, VecDeque<String>>,
}

impl MockTerminal {
    pub fn new() -> Self {
        Self { outputs: std::collections::HashMap::new() }
    }
    
    pub async fn next_output_opt(&mut self, id: SessionId) -> anyhow::Result<Option<String>> {
        Ok(self.outputs.get_mut(&id).and_then(|q| q.pop_front()))
    }
}

#[async_trait]
impl TerminalBackend for MockTerminal {
    async fn spawn(&mut self, _cmd: &str) -> anyhow::Result<SessionId> {
        let id = Uuid::new_v4();
        let mut queue = VecDeque::new();
        queue.push_back("hello".to_string());
        self.outputs.insert(id, queue);
        Ok(id)
    }
    
    async fn write(&mut self, _id: SessionId, _input: &str) -> anyhow::Result<()> {
        Ok(())
    }
    
    async fn kill(&mut self, id: SessionId) -> anyhow::Result<()> {
        self.outputs.remove(&id);
        Ok(())
    }
}

pub struct Link { pub range: (usize, usize), pub url: String }

pub fn detect_links(s: &str) -> Vec<Link> {
    // Simple URL detection (http/https)
    let mut links = Vec::new();
    let mut in_url = false;
    let mut start = 0;
    let chars: Vec<char> = s.chars().collect();
    
    for (i, &c) in chars.iter().enumerate() {
        if !in_url && i + 7 < chars.len() && &chars[i..i+7].iter().collect::<String>() == "http://" {
            in_url = true;
            start = i;
        } else if in_url && (c == ' ' || c == '\n') {
            in_url = false;
            links.push(Link { range: (start, i), url: chars[start..i].iter().collect() });
        }
    }
    
    links
}
