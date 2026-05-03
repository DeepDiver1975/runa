use runa::core::executor::Executor;
use runa::plan::parser::parse_plan;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let plan_path = args
        .iter()
        .position(|a| a == "--plan")
        .and_then(|i| args.get(i + 1))
        .expect("--plan <path>");
    let md = fs::read_to_string(plan_path)?;
    let steps = parse_plan(&md)?;
    if steps.is_empty() {
        eprintln!("no steps found in plan: {}", plan_path);
        std::process::exit(2);
    }
    let mut exec = Executor::new()?;
    let res = exec.run(&steps[0])?;
    println!("{}", res.output);
    if res.status != runa::core::executor::StepStatus::Done {
        std::process::exit(1);
    }
    Ok(())
}
