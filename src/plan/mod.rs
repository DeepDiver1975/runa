use pulldown_cmark::{Parser, Event, Tag};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Plan { pub steps: Vec<crate::state::Step> }

pub fn parse_plan_from_markdown(md: &str) -> Result<Plan, &'static str> {
    let parser = Parser::new(md);
    let mut in_code = false;
    let mut code_buf = String::new();
    let mut steps = Vec::new();

    for ev in parser {
        match ev {
            Event::Start(Tag::CodeBlock(_)) => { in_code = true; code_buf.clear(); }
            Event::Text(text) if in_code => { code_buf.push_str(&text); }
            Event::End(Tag::CodeBlock(_)) => {
                let step = crate::state::Step {
                    id: Uuid::new_v4(),
                    title: "".into(),
                    description: "".into(),
                    code: code_buf.clone(),
                    status: crate::state::StepStatus::Pending,
                };
                steps.push(step);
                in_code = false;
            }
            _ => {}
        }
    }

    Ok(Plan { steps })
}
