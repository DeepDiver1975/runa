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
    let mut in_heading = false;
    let mut heading_buf = String::new();
    let mut code_buf = String::new();
    let mut desc_buf = String::new();
    let mut last_heading: Option<String> = None;
    let mut steps = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading(..)) => {
                in_heading = true;
                heading_buf.clear();
            }
            Event::End(Tag::Heading(..)) => {
                in_heading = false;
                last_heading = Some(heading_buf.trim().to_string());
            }
            Event::Start(Tag::CodeBlock(_info)) => {
                in_code = true;
                code_buf.clear();
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code = false;
                steps.push(Step {
                    title: last_heading.clone().unwrap_or_default(),
                    description: desc_buf.trim().to_string(),
                    code: code_buf.clone(),
                });
                desc_buf.clear();
            }
            Event::Text(t) => {
                if in_heading {
                    heading_buf.push_str(&t);
                } else if in_code {
                    code_buf.push_str(&t);
                } else {
                    // accumulate description text between headings/code blocks
                    desc_buf.push_str(&t);
                    desc_buf.push(' ');
                }
            }
            _ => {}
        }
    }

    Ok(steps)
}
