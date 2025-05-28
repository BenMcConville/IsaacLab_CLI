use super::Mainpage;
use ratatui::{
    Terminal,
    backend::{self, CrosstermBackend},
    layout::Alignment,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Backend,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};
use serde_yaml::Value; // Make sure to import Value

// Helper function to convert Value to lines for display
fn yaml_to_lines(yaml_value: &Value) -> Vec<String> {
    // Pretty-print the YAML value to a string
    serde_yaml::to_string(yaml_value)
        .unwrap_or_else(|_| "Error formatting YAML".to_string())
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn render_yaml_page_ui<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mp_struct: &Mainpage,
) {
    terminal
        .draw(|f| {
            let size = f.size();

            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(size);

            // --- Left side (Task Queue) ---
            let task_names = mp_struct.get_task_queue_names();
            let left_block = Block::default().borders(Borders::ALL).title("Task Queue");

            let selected_task_index = Some(mp_struct.task_selection + 1); // +1 because of blank line

            let task_items: Vec<ListItem> = {
                let mut items = vec![ListItem::new(" ")]; // blank line after title
                items.extend(
                    task_names
                        .iter()
                        .map(|task| ListItem::new(Span::raw(format!("    {}", task)))),
                );
                items
            };

            let selected_color = {
                if *mp_struct.get_active_view() {
                    Color::Green
                } else {
                    Color::Yellow
                }
            };
            let task_list = List::new(task_items)
                .block(left_block)
                .highlight_symbol(" > ")
                .highlight_style(
                    Style::default()
                        .fg(selected_color)
                        .add_modifier(Modifier::BOLD),
                );

            let mut task_list_state = ListState::default();
            task_list_state.select(selected_task_index);

            f.render_stateful_widget(task_list, layout[0], &mut task_list_state);

            // --- Right side (YAML Display) ---
            let yaml_lines = if let Some(yaml_value) = &mp_struct.temp_yaml {
                yaml_to_lines(yaml_value)
            } else {
                vec!["No YAML data loaded.".to_string()]
            };

            let yaml_items: Vec<ListItem> = yaml_lines
                .into_iter()
                .map(|line| ListItem::new(Span::raw(line)))
                .collect();

            let mut yaml_list_state = ListState::default();
            if mp_struct.yaml_selection < yaml_items.len() {
                yaml_list_state.select(Some(mp_struct.yaml_selection));
            }

            let yaml_list = List::new(yaml_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("YAML Configuration"),
                )
                .highlight_symbol(">>")
                .highlight_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                );

            f.render_stateful_widget(yaml_list, layout[1], &mut yaml_list_state);

            // --- Popup Display ---
            if mp_struct.update_yaml_selection {
                // Define the block for the popup
                let popup_block = Block::default().title("Update Field").borders(Borders::ALL);

                // Calculate the area for the popup (e.g., centered)
                let popup_area = centered_rect(60, 20, size); // 60% width, 20% height of parent area

                // Render the background clear for the popup
                f.render_widget(Clear, popup_area);
                // Render the popup block - PASS A REFERENCE HERE!
                f.render_widget(&popup_block, popup_area); // <-- CHANGE HERE

                // Get the inner area of the block
                let inner_area = popup_block.inner(popup_area); // This now works because popup_block was only borrowed

                // Create the paragraph widget for the text content
                let text = Paragraph::new(mp_struct.yaml_update_text.as_str())
                    .alignment(Alignment::Center); // Centered text inside the inner area

                // Render the text into the inner area
                f.render_widget(text, inner_area);
            }
        })
        .unwrap();
}

/// Helper function to create a rect in the middle of the given area
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
