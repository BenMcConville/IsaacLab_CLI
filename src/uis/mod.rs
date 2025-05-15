use ratatui::{
    Terminal,
    backend::{self, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    prelude::Backend,
    style::{Color, Style},
    text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render_main_page_ui<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) {
    terminal
        .draw(|f| {
            let size = f.size();

            // Define layout with two columns
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(size);

            // Left Column - Block
            let left_block = Block::default().borders(Borders::ALL).title("Task Queue");

            // Right Column - Block with Text Box
            let right_block = Block::default().borders(Borders::ALL).title("Options");

            // Render the blocks
            f.render_widget(left_block, layout[0]);
            f.render_widget(right_block, layout[1]);

            // // Create text input for the right column
            // let paragraph = Paragraph::new(Text::from(app.input.as_str()))
            //     .block(right_block)
            //     .style(Style::default().fg(Color::White));

            // // Render the right block (text box)
            // f.render_widget(paragraph, layout[1]);
        })
        .unwrap();
}
