use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{app::App, tui::Frame};

pub fn render(app: &mut App, f: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(f.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layout[1]);

    let title_block = Paragraph::new(format!(
        "Press `:q` to exit.\n Use hotkeys to enter a calculation string.\n"
    ))
    .block(
        Block::default()
            .title("Commandline Calculator")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default())
    .alignment(Alignment::Center);

    let current_block = Paragraph::new(format!("{}", app.current_calc))
        .block(
            Block::default()
                .title("Current Calculation")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default())
        .alignment(Alignment::Right);

    let history_block = Paragraph::new(format!("{:?}", app.history))
        .block(
            Block::default()
                .title("Calculation History")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default())
        .alignment(Alignment::Right);

    f.render_widget(title_block, outer_layout[0]);
    f.render_widget(current_block, inner_layout[0]);
    f.render_widget(history_block, inner_layout[1]);
}
