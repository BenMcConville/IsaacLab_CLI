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

            let task_names = mp_struct.get_task_queue_names();
            let left_block = Block::default().borders(Borders::ALL).title("Task Queue");

            let selected_index = Some(mp_struct.task_selection + 1);

            let task_items: Vec<ListItem> = {
                let mut items = vec![ListItem::new(" ")]; // blank line after title
                items.extend(
                    task_names
                        .iter()
                        .map(|task| ListItem::new(Span::raw(format!("   {}", task)))),
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

            let mut state = ListState::default();
            state.select(selected_index);

            f.render_stateful_widget(task_list, layout[0], &mut state);

            // Right side (optional placeholder block)
            let right_block = Block::default().borders(Borders::ALL).title("Options");
            f.render_widget(right_block, layout[1]);
        })
        .unwrap();
}
