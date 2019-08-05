use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Gauge, Sparkline, Widget};
use tui::Terminal;

use super::models::{MemorySnapshot, SystemReport};

fn memory_gauge_label<'a>(memory: &MemorySnapshot) -> String {
    format!(
        "{}/{} ({}%)",
        memory.get_used_display(),
        memory.get_total_display(),
        memory.get_used_percentage()
    )
}

fn render_memory_gauge<'a, B: Backend>(memory: &MemorySnapshot, frame: &mut Frame<B>, area: Rect) {
    Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow))
        .label(&memory_gauge_label(memory))
        .percent(memory.get_used_percentage())
        .render(frame, area);
}

fn get_memory_usage_history_data(memory_histories: Vec<&MemorySnapshot>) -> Vec<u64> {
    memory_histories
        .iter()
        .map(|mh| mh.get_used_bytes())
        .collect()
}
fn render_memory_history<B: Backend>(
    memory_histories: Vec<&MemorySnapshot>,
    mut frame: &mut Frame<B>,
    area: Rect,
) {
    Sparkline::default()
        .block(
            Block::default()
                .title("Memory Usage History")
                .borders(Borders::LEFT | Borders::RIGHT),
        )
        .data(&get_memory_usage_history_data(memory_histories))
        .style(Style::default().fg(Color::Green))
        .render(&mut frame, area)
}

pub fn render<B: Backend>(
    terminal: &mut Terminal<B>,
    reports: &Vec<SystemReport>,
) -> Result<(), std::io::Error> {
    terminal.draw(|mut frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(frame.size());

        // Current state 
        match reports.first() {
            Some(report) => render_memory_gauge(report.get_memory(), &mut frame, chunks[0]),
            None => (),
        }

        let memory_history = reports.iter().map(|r| r.get_memory()).collect();

        // Historical State
        render_memory_history(memory_history, &mut frame, chunks[1]);
    })
}
