use pulldown_cmark::{Event, Parser, Tag};

#[derive(Debug, Clone)]
pub struct Step {
    pub title: String,
    pub description: String,
    pub code: String,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("markdown parse error")]
    Parse,
}

pub fn parse_plan(md: &str) -> Result<Vec<Step>, ParseError> {
    let parser = Parser::new(md);
    let mut in_code = false;
    let mut code_buf = String::new();
    let mut steps = Vec::new();
    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_info)) => {
                in_code = true;
                code_buf.clear();
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code = false;
                steps.push(Step {
                    title: String::new(),
                    description: String::new(),
                    code: code_buf.clone(),
                });
            }
            Event::Text(t) => {
                if in_code {
                    code_buf.push_str(&t);
                }
            }
            _ => {}
        }
    }
    Ok(steps)
}
