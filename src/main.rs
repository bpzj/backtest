#![windows_subsystem = "windows"]
mod account;
mod model;
mod strategy;
mod ui;

fn main() -> Result<(), eframe::Error> {
    ui::run_app()
}
