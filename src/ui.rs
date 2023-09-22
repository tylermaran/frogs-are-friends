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
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50), // Allocates 50% width to the first block
                Constraint::Percentage(50), // Allocates 50% width to the second block
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
        .style(Style::default().fg(Color::Green).bg(Color::Black))
        .alignment(Alignment::Left),
        chunks[0],
    );

    // Block for Planet Y (just a sample, you can modify it as needed)
    frame.render_widget(
        Paragraph::new(format!(
            "Manufacturing Overview for Planet Y.\n\
            Metal Production: {}\n\
            Energy Production: {}\n\
            Food Production: {}",
            app.metal_production, app.energy_production, app.food_production
        ))
        .block(
            Block::default()
                .title("<Planet Y>")
                .padding(Padding::new(2, 2, 1, 1))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .style(Style::default().fg(Color::Green).bg(Color::Black))
        .alignment(Alignment::Left),
        chunks[1],
    );
}
