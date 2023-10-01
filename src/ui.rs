use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(90), // Allocates 90% height to the main area.
                Constraint::Percentage(10), // Allocates 10% height to the input field.
            ]
            .as_ref(),
        )
        .split(frame.size());

    // Block for Planet X
    frame.render_widget(
        Paragraph::new(format!(
            "Manufacturing Overview for Planet X.\n\
            Metal Production: {}\n\
            Energy Production: {}\n\
            Food Production: {}",
            app.metal_production, app.energy_production, app.food_production
        ))
        .block(
            Block::default()
                .title("<Planet X>")
                .padding(Padding::new(2, 2, 1, 1))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Left),
        chunks[0],
    );

    // Input field
    let input_area = chunks[1];
    let input_text = Paragraph::new(app.input.as_ref() as &str)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    frame.render_widget(input_text, input_area);
}
