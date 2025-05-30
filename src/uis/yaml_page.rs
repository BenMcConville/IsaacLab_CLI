use super::Mainpage;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::Alignment,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};
use serde_yaml::Value;
use std::fmt::Write;

// Helper function to convert Value to lines for display.
fn yaml_to_lines(yaml_value: &Value) -> Vec<String> {
    let mut lines = Vec::new();
    traverse_yaml(yaml_value, &mut lines, 0);
    lines
}

fn traverse_yaml(value: &Value, lines: &mut Vec<String>, indent_level: usize) {
    let indent = "    ".repeat(indent_level); // 4 spaces per indent level

    match value {
        Value::Mapping(map) => {
            for (key, val) in map {
                let key_str = key.as_str().unwrap_or_default();
                let mut line = String::new();

                match val {
                    Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => {
                        // If the value is a scalar, format and add the key-value pair.
                        write!(&mut line, "{indent}{key_str}: ").unwrap();
                        match serde_yaml::to_string(val) {
                            Ok(s) => {
                                line.push_str(s.trim_end());
                            }
                            Err(_) => {
                                line.push_str(&format!("{:?}", val));
                            }
                        }
                        lines.push(line);
                    }
                    Value::Sequence(sequence) => {
                        // If the value is a sequence, print the key and then
                        // recurse for each item with increased indent.
                        write!(&mut line, "{indent}{key_str}:").unwrap(); // Add the key for the sequence
                        lines.push(line);
                        for item in sequence {
                            traverse_yaml(item, lines, indent_level + 1);
                        }
                    }
                    Value::Mapping(_) => {
                        // If the value is a nested map, you explicitly want to skip adding
                        // a line for this key. Just recurse.
                        traverse_yaml(val, lines, indent_level + 1);
                    }
                    Value::Tagged(tagged_value) => {
                        // For a tagged value, check its inner type.
                        match &tagged_value.value {
                            Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => {
                                // If the inner tagged value is scalar, print the key and the scalar.
                                write!(&mut line, "{indent}{key_str}: ").unwrap();
                                match serde_yaml::to_string(val) {
                                    // Use `val` here to get the full tagged representation if needed
                                    Ok(s) => line.push_str(s.trim_end()),
                                    Err(_) => line.push_str(&format!("{:?}", val)),
                                }
                                lines.push(line);
                            }
                            Value::Sequence(_) => {
                                // If the inner tagged value is a sequence, print the key and then recurse.
                                write!(&mut line, "{indent}{key_str}:").unwrap();
                                lines.push(line);
                                traverse_yaml(&tagged_value.value, lines, indent_level + 1);
                            }
                            Value::Mapping(_) => {
                                // If the inner tagged value is a mapping, skip the key and recurse.
                                traverse_yaml(&tagged_value.value, lines, indent_level + 1);
                            }
                            // FIX: Add this branch to handle nested tagged values
                            Value::Tagged(_) => {
                                write!(&mut line, "{indent}{key_str}:").unwrap(); // Print key if it's a tagged complex type
                                lines.push(line);
                                traverse_yaml(&tagged_value.value, lines, indent_level + 1);
                            }
                        }
                    }
                }
            }
        }
        Value::Sequence(sequence) => {
            // This case handles sequences that are either top-level or elements within another sequence.
            for item in sequence {
                let mut line = String::new();
                match item {
                    Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => {
                        // If a sequence item is a scalar, display it with a hyphen.
                        write!(&mut line, "{indent}- ").unwrap();
                        match serde_yaml::to_string(item) {
                            Ok(s) => {
                                line.push_str(s.trim_end());
                            }
                            Err(_) => {
                                line.push_str(&format!("{:?}", item));
                            }
                        }
                        lines.push(line);
                    }
                    Value::Mapping(_) | Value::Sequence(_) => {
                        // If a sequence item is a Mapping or Sequence, skip showing the hyphen for the parent,
                        // and recurse into the child to display its scalar children.
                        traverse_yaml(item, lines, indent_level + 1);
                    }
                    Value::Tagged(tagged_item) => {
                        // Handle tagged items within a sequence.
                        match &tagged_item.value {
                            Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => {
                                // If the inner tagged value is scalar, display with hyphen.
                                write!(&mut line, "{indent}- ").unwrap();
                                match serde_yaml::to_string(item) {
                                    Ok(s) => line.push_str(s.trim_end()),
                                    Err(_) => line.push_str(&format!("{:?}", item)),
                                }
                                lines.push(line);
                            }
                            // FIX: Add this branch to handle nested tagged values within a sequence
                            Value::Tagged(_) | Value::Mapping(_) | Value::Sequence(_) => {
                                // If complex tagged item, just recurse.
                                // If you want a hyphen before the nested structure here, add:
                                // write!(&mut line, "{indent}- ").unwrap(); lines.push(line);
                                traverse_yaml(&tagged_item.value, lines, indent_level + 1);
                            }
                        }
                    }
                }
            }
        }
        // Handle the top-level value if it's a scalar.
        Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => {
            if indent_level == 0 {
                lines.push(
                    serde_yaml::to_string(value)
                        .unwrap_or_else(|_| "Error formatting scalar".to_string())
                        .trim_end()
                        .to_string(),
                );
            }
        }
        Value::Tagged(tagged_value) => {
            // If the top-level value is tagged, recurse into its inner value.
            if indent_level == 0 {
                traverse_yaml(&tagged_value.value, lines, indent_level); // Keep same indent level for inner value
            }
        }
    }
}

pub fn render_yaml_page_ui<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mp_struct: &Mainpage,
) {
    terminal
        .draw(|f| {
            let size = f.area();

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
                // Render the popup block
                f.render_widget(&popup_block, popup_area);

                // Get the inner area of the block
                let inner_area = popup_block.inner(popup_area);

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
