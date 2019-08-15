use tui::backend::Backend;
use tui::layout::{Layout, Constraint, Direction};
use tui::Terminal;

mod historical_gauge;

use crate::models::{ProcessorSnapshot, SystemReport};

/*
fn format_disks_for_list(disks: &[DiskSnapshot]) -> Vec<Text> {
    disks
        .iter()
        .map(|d| Text::styled(d.description(), Style::default().fg(Color::Magenta)))
        .collect()
}
fn render_disk_list<B: Backend>(disks: &[DiskSnapshot], frame: &mut Frame<B>, area: Rect) {
    let texts: std::iter::Iterator::Item == tui::widgets::Text<'static> = format_disks_for_list(disks).iter();
    List::new(texts)
        .block(Block::default().borders(Borders::ALL).title("Disks"))
        .start_corner(Corner::TopLeft)
        .render(&mut frame, area)
}
*/

fn read_first_snapshot(report: &SystemReport) -> std::option::Option<(std::time::SystemTime, ProcessorSnapshot)>{
    match report.get_processors().first() {
        Some(processor) => Some((*report.get_time(), processor.clone())),
        None => None,
    }
}

pub fn render<B: Backend>(
    terminal: &mut Terminal<B>,
    reports: &Vec<SystemReport>,
) -> Result<(), std::io::Error> {
    terminal.draw(|mut frame| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ].as_ref())
            .split(frame.size());

        let memory_history_tuple = reports.iter()
            .map(|report| (*report.get_time(), report.get_memory().clone()))
            .collect();
        historical_gauge::render_historical_gauge(memory_history_tuple, &mut frame, chunks[0]);

        let processor_history_tuple = reports.iter()
            .filter_map(|report|read_first_snapshot(report))
            .collect();

        historical_gauge::render_historical_gauge(processor_history_tuple, &mut frame, chunks[1]);
    })
}
