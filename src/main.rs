mod app;
mod core;
mod plan;
mod terminal;
mod ui;
mod util;

fn main() {
    println!("Runa scaffold initialized.");
    // Call module inits to avoid dead-code warnings in early development
    app::init();
    core::init();
    ui::init();
    plan::init();
    util::init();
}
