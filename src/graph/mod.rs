extern crate tui;

use std::io;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Gauge, Widget};
use tui::{Frame, Terminal};

pub fn draw<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.size());
        draw_first_tab(&mut f, chunks[1]);
    })
}

fn draw_first_tab<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(7),
                Constraint::Min(7),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    draw_gauges(f, chunks[0]);
}

fn draw_gauges<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(2), Constraint::Length(3)].as_ref())
        .margin(1)
        .split(area);
    Block::default()
        .borders(Borders::ALL)
        .title("Graphs")
        .render(f, area);
    Gauge::default()
        .block(Block::default().title("Gauge:"))
        .style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .label(&format!("{} / 100", 65))
        .percent(65)
        .render(f, chunks[0]);
}
