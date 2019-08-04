use tui::widgets::{Block,Borders,Gauge, Widget};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color,Style};
use tui::backend::Backend;
use tui::terminal::Frame;
use tui::Terminal;

use super::models::{MemorySnapshot, SystemReport};

fn memory_gauge_label<'a>(memory: &MemorySnapshot) -> String {
    format!("{}/{} ({}%)", memory.get_used_display(), memory.get_total_display(), memory.get_used_percentage())
}

fn render_memory_gauge<'a, B: Backend>(memory: &MemorySnapshot, frame: &mut Frame<B>, area: Rect) {
    Gauge::default()
        .block(Block::default().title("Memory").borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow))
        .label(&memory_gauge_label(memory))
        .percent(memory.get_used_percentage())
        .render(frame, area);
}

pub fn render<B: Backend>(terminal: &mut Terminal<B>, reports: &Vec<SystemReport>) -> Result<(), std::io::Error> {
    terminal.draw(|mut frame| {
        match reports.first() {
            Some(report) => {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]
                .as_ref()
            )
            .split(frame.size());
        render_memory_gauge(report.get_memory(), &mut frame, chunks[0]);
            }
            None => (),
        }
    })

}
