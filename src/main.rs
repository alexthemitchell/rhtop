extern crate crossterm;

mod display;
mod models;
mod system;

use std::{thread, time};

use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

static TWO_SECONDS: time::Duration = time::Duration::from_secs(2);

fn main() {
    let mut backend = CrosstermBackend::new();
    match backend.clear() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    let term = Terminal::new(backend);
    match term {
        Ok(terminal) => {
            report_loop(terminal);
        }
        Err(e) => println!("{}", e),
    }
}

fn report_loop<B: Backend>(mut terminal: Terminal<B>) {
    let mut reports = Vec::new();
    reports.push(system::new_report());
    loop {
        reports.push(system::new_report());
        match display::render(&mut terminal, &reports) {
            Ok(_) => thread::sleep(TWO_SECONDS),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}
