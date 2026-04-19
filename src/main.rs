mod app;
mod core;
mod ui;
mod terminal;
mod plan;
mod util;

fn main() {
    println!("Runa scaffold initialized.");
    // Call module inits to avoid dead-code warnings in early development
    app::init();
    core::init();
    ui::init();
    terminal::init();
    plan::init();
    util::init();
}
