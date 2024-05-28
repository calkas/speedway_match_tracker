use crate::smt_app::App;
use ratatui::prelude::*;
use ratatui::widgets::block::Position;
use ratatui::widgets::{block::Title, Block, Borders, Paragraph};

pub fn render(app: &mut App, frame: &mut Frame) {
    main_ui(frame);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(frame.size());

    match_calendar_ui(app.data.match_information.clone(), frame, layout[0]);

    match_table_ui(
        "PGE Ekstraliga".to_owned(),
        app.data.table_super_league.clone(),
        frame,
        layout[1],
    );

    match_table_ui(
        "Metalkas 2. Ekstraliga".to_owned(),
        app.data.table_1_league.clone(),
        frame,
        layout[2],
    );
}

fn main_ui(frame: &mut Frame) {
    let app_name = Title::from("Speedway Match Tracker".blue().bold());
    let instructions = Title::from(Line::from(vec![" Quit ".into(), "<q> ".blue().bold()]));

    let header = Block::default()
        .borders(Borders::ALL)
        .title(app_name.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        );

    frame.render_widget(header, frame.size());
}

fn match_calendar_ui(table_content: String, frame: &mut Frame, layout: Rect) {
    frame.render_widget(
        Paragraph::new(table_content).block(
            Block::new()
                .title("Match calendar")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL),
        ),
        layout,
    );
}

fn match_table_ui(name: String, table_content: String, frame: &mut Frame, layout: Rect) {
    frame.render_widget(
        Paragraph::new(table_content).block(
            Block::new()
                .title(name)
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL),
        ),
        layout,
    );
}
