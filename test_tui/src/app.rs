use std::time::{Duration, Instant};

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self, Marker},
    text::{Line, Span},
    widgets::{Axis, Block, Chart, Dataset, GraphType, LegendPosition},
    DefaultTerminal, Frame,
};


#[derive(Clone)]
pub struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64
}

impl SinSignal {
    pub fn new(interval: f64, period: f64, scale: f64) -> Self {
        Self {
            x: 0.0,
            interval,
            period,
            scale
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let pnt = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(pnt)
    }
}

pub struct App {
    s1: SinSignal,
    d1: Vec<(f64, f64)>,
    s2: SinSignal,
    d2: Vec<(f64, f64)>,
    w: [f64; 2],
}

impl App {
    pub fn new() -> Self {
        let mut s1 = SinSignal::new(0.2, 3.0, 18.0);
        let mut s2 = SinSignal::new(0.1, 2.0, 10.0);

        let d1 = s1.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        let d2 = s2.by_ref().take(200).collect::<Vec<(f64, f64)>>();

        Self {
            s1,
            d1,
            s2,
            d2,
            w: [0.0, 20.0]
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tr = Duration::from_millis(500);

        let mut lt = Instant::now();

        loop {
            terminal.draw(|f| self.draw(f))?;

            let to = tr.saturating_sub(lt.elapsed());

            // Poll for events.
            if event::poll(to)? {
                // Read the event and if it's a key event, start looking at the key codes.
                if let Event::Key(key) = event::read()? {
                    let key = key.code;

                    match key {
                        KeyCode::Char('q') => break Ok(()),
                        _ => {}
                    }
                }
            }

            // Check if we need to tick.
            if lt.elapsed() >= tr {
                self.on_tick();

                // We need to reset the last tick time so we know when to tick again.
                lt = Instant::now();
            }
        }
    }

    fn on_tick(&mut self) {
        self.d1.drain(0..5);
        self.d1.extend(self.s1.by_ref().take(5));

        self.d2.drain(0..10);
        self.d2.extend(self.s2.by_ref().take(10));

        self.w[0] += 1.0;
        self.w[1] += 1.0;
    }

    fn draw(&self, f: &mut Frame) {
        let area = f.area();

        let net_chart = Layout::horizontal([Constraint::Percentage(100)]).split(area);
        
        self.render_chart(f, net_chart[0]);
    }

    fn render_chart(&self, f:  &mut Frame, a: Rect) {
        let x_labels = vec![
            Span::styled(format!("{}", self.w[0]), Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}", (self.w[0] + self.w[1]) / 2.0)),
            Span::styled(format!("{}", self.w[1]), Style::default().add_modifier(Modifier::BOLD)),
        ];

        let ds = vec![
            Dataset::default()
                .name("data2")
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Cyan))
                .data(&self.d1),
            Dataset::default()
                .name("data3")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .data(&self.d2)
        ];

        let chart = Chart::new(ds)
            .block(Block::bordered())
            .x_axis(
                Axis::default()
                    .title("X Axis")
                    .style(Style::default().fg(Color::Gray))
                    .bounds(self.w)
                    .labels(x_labels)
            )
            .y_axis(
                Axis::default()
                    .title("Y Axis")
                    .style(Style::default().fg(Color::Gray))
                    .labels(["-20".bold(), "0".into(), "20".bold()])
                    .bounds([-20.0, 20.0]),
            );

            f.render_widget(chart, a);
    }
}