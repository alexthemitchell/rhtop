use tui::backend::Backend;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, Marker, Widget};
use tui::layout::{Layout,Direction, Constraint, Rect};
use tui::style::{Color, Style};
use tui::terminal::Frame;
use crate::models::{Gaugable};

pub fn render_historical_gauge<B: Backend, G: Gaugable>(
    historical_reports: Vec<(std::time::SystemTime,G)>,
    frame: &mut Frame<B>,
    area: Rect,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(area.inner(0));
    match historical_reports.last() {
        Some((_,report)) => {
            render_gauge(report, frame, chunks[0]);
        },
        None => (),
    }

    render_gauge_history("test", historical_reports, frame, chunks[1]);

}

fn render_gauge<'a, B: Backend, G: Gaugable>(gauge: &G, frame: &mut Frame<B>, area: Rect) {
    Gauge::default()
        .block(
            Block::default()
                .title(&gauge.get_name())
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::Yellow))
        .label(&gauge.display().to_string())
        .ratio(gauge.get_percentage() / 100.0)
        .render(frame, area);
}

fn data_representation_of_time(time: &std::time::SystemTime) -> f64 {
    match time.duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs() as f64,
        Err(_) => 0.0,
    }
}

fn get_historical_gauge_used_data<G: Gaugable>(historical_reports: Vec<(std::time::SystemTime,G)>) -> Vec<(f64, f64)> {
    historical_reports
        .iter()
        .map(|(time, hr)| {
            (
                data_representation_of_time(time),
                hr.get_percentage(),
            )
        })
        .collect()
}

fn get_bounds_for_data(data: &Vec<(f64, f64)>) -> ((f64, f64), (f64, f64)) {
    data.iter().fold(
        ((std::f64::MAX, 0.0), (std::f64::MAX, 0.0)),
        |((xmin, xmax), (ymin, ymax)), (x, y)| {
            ((x.min(xmin), x.max(xmax)), (y.min(ymin), y.max(ymax)))
        },
    )
}


fn render_gauge_history<B: Backend, G: Gaugable>(
    gauge_name: &str,
    historical_reports: Vec<(std::time::SystemTime,G)>,
    mut frame: &mut Frame<B>,
    area: Rect,
) {
    let data = &get_historical_gauge_used_data(historical_reports);
    let ((xmin, xmax), _) = get_bounds_for_data(data);
    Chart::default()
        .block(
            Block::default()
                .title(gauge_name)
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("Time")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::White))
                .bounds([xmin, xmax])
                .labels(&[&format!("{}", xmin), &format!("{}", xmax)]),
        )
        .y_axis(
            Axis::default()
                .title("Percent")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::White))
                .bounds([0.0, 100.0])
                .labels(&["0%", "25%", "50%", "75%", "100%"]),
        )
        .datasets(&[Dataset::default()
            .name("Used")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .data(data)])
        .render(&mut frame, area)
}
