use std::sync::Arc;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, Wrap},
    Terminal,
};

use crate::state::WorldState;

pub async fn run_dashboard(world: Arc<WorldState>) {
    enable_raw_mode().expect("failed to enable raw mode");
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).expect("failed to enter alternate screen");

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("failed to create terminal");

    let mut alert_buffer = vec![
        "System online".to_string(),
        "Monitoring 100 drones".to_string(),
        "Waiting for telemetry".to_string(),
    ];

    loop {
        let rows: Vec<_> = world
            .drones
            .iter()
            .map(|entry| {
                let telemetry = entry.value();
                Row::new(vec![
                    Cell::from(telemetry.drone_id.to_string()),
                    Cell::from(format!("{:.2}", telemetry.lat)),
                    Cell::from(format!("{:.2}", telemetry.lon)),
                    Cell::from(format!("{:.2}", telemetry.speed)),
                    Cell::from(format!("{:.1}", telemetry.battery_pct)),
                    Cell::from(if telemetry.is_armed { "ARMED" } else { "SAFE" }),
                ])
            })
            .take(100)
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(8),
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(8),
            ],
        )
        .header(
            Row::new(vec!["Drone", "Lat", "Lon", "Speed", "Batt%", "State"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        )
        .block(Block::default().title("Telemetry Fleet").borders(Borders::ALL))
        .widths(&[
            Constraint::Length(8),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(8),
        ]);

        terminal
            .draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                    .split(frame.size());

                let main = chunks[0];
                let alert_area = chunks[1];
                frame.render_widget(table, main);

                let lines = alert_buffer
                    .iter()
                    .rev()
                    .take(8)
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>();
                let alert_list = ratatui::widgets::Paragraph::new(lines.join("\n"))
                    .wrap(Wrap { trim: true })
                    .block(Block::default().title("Alerts").borders(Borders::ALL));
                frame.render_widget(alert_list, alert_area);
            })
            .expect("failed to render dashboard");

        tokio::time::sleep(std::time::Duration::from_millis(250)).await;

        if world.drones.is_empty() {
            alert_buffer.push("No telemetry received".to_string());
        }

        if alert_buffer.len() > 20 {
            alert_buffer.remove(0);
        }

        if !world.drones.is_empty() {
            alert_buffer.push(format!("Live drones: {}", world.drones.len()));
        }
    }
}

pub fn shutdown_terminal() {
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, LeaveAlternateScreen);
    let _ = disable_raw_mode();
}
