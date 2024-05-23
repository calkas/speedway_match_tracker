use crate::smt_app::App;

use crossterm::style::Stylize;
use ratatui::{
    layout::{self, Constraint, Direction, Layout},
    prelude::{Alignment, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{block::Title, Block, Borders, Paragraph},
    Frame,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let header = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Speedway Match Tracker").alignment(Alignment::Center))
        .title_style(Style::default().fg(Color::Cyan));

    frame.render_widget(header, frame.size());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new(app.data.match_information.clone()).block(
            Block::new()
                .title("Match Information")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        ),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new(app.data.table_super_league.clone()).block(
            Block::new()
                .title("Super League Table")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        ),
        layout[1],
    );

    frame.render_widget(
        Paragraph::new(app.data.table_1_league.clone()).block(
            Block::new()
                .title("1 League Table")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        ),
        layout[2],
    );
}
