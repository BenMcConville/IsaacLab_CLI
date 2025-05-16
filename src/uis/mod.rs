use ratatui::{
    Terminal,
    backend::{self, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    prelude::Backend,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub struct Mainpage {
    task_selection: usize,
    task_list: Vec<String>,
}
impl Mainpage {
    pub fn new() -> Self {
        Self {
            task_selection: 0,
            task_list: vec![
                String::from("test01"),
                String::from("test02"),
                String::from("test03"),
            ],
        }
    }
    pub fn get_task_queue_names(&self) -> Vec<&str> {
        let mut task_queue_names: Vec<&str> = vec![];
        for task in &self.task_list {
            task_queue_names.push(task);
        }
        task_queue_names
    }
    pub fn increase_selection(&mut self) {
        let len = self.task_list.len();

        if len == 0 {
            self.task_selection = 0; // no items to select
        } else {
            // Wrap around from last to first (0-based)
            self.task_selection = (self.task_selection + 1) % len;
        }
    }

    pub fn decrease_selection(&mut self) {
        let len = self.task_list.len();

        if len == 0 {
            self.task_selection = 0;
        } else {
            // Wrap around from first to last, careful with underflow
            if self.task_selection == 0 {
                self.task_selection = len - 1;
            } else {
                self.task_selection -= 1;
            }
        }
    }
}

pub fn render_main_page_ui<B: ratatui::backend::Backend>(
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

            let task_list = List::new(task_items)
                .block(left_block)
                .highlight_symbol(" > ")
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
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
