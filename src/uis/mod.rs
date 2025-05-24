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

pub struct Mainpage {
    task_selection: usize,
    active_view: bool,
    task_list: Vec<String>,
    task_running: bool,
    task_execution_location: String,
    wandb: bool,
    create_window: bool,
    create_task_name: String,
    create_env: String,
    create_dir: String,
}
impl Mainpage {
    pub fn new() -> Self {
        Self {
            task_selection: 0,
            active_view: false,
            task_list: vec![],
            task_running: false,
            task_execution_location: String::from("LOCAL"),
            wandb: false,
            create_window: false,
            create_task_name: String::from(""),
            create_env: String::from(""),
            create_dir: String::from(""),
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
    pub fn update_temp_task(&mut self, task_name: &str, env_name: &str, dir: &str) {
        self.create_task_name = String::from(task_name);
        self.create_env = String::from(env_name);
        self.create_dir = String::from(dir);
    }
    pub fn get_current_task_selection_name(&self) -> &str {
        &self.task_list[self.task_selection]
    }

    pub fn get_temp_name(&self) -> &str {
        &self.create_task_name
    }
    pub fn get_temp_env(&self) -> &str {
        &self.create_env
    }
    pub fn get_temp_dir(&self) -> &str {
        &self.create_dir
    }
    pub fn set_temp_name<S: Into<String>>(&mut self, name: S) {
        self.create_task_name = name.into();
    }

    pub fn set_temp_env<S: Into<String>>(&mut self, env: S) {
        self.create_env = env.into();
    }

    pub fn set_temp_dir<S: Into<String>>(&mut self, dir: S) {
        self.create_dir = dir.into();
    }
    pub fn get_task_running(&self) -> &bool {
        &self.task_running
    }
    pub fn update_task_list(&mut self, task_list: Vec<&str>) {
        self.task_list.clear();
        for task_name in task_list {
            self.task_list.push(String::from(task_name));
        }
    }
    pub fn get_active_view(&self) -> &bool {
        &self.active_view
    }
    pub fn get_wandb(&self) -> &bool {
        &self.wandb
    }
    pub fn get_task_execution_location(&self) -> &str {
        &self.task_execution_location
    }

    pub fn set_active_view(&mut self, set_val: bool) {
        self.active_view = set_val;
    }
    pub fn set_create_window(&mut self, set_val: bool) {
        self.create_window = set_val;
    }

    pub fn get_create_window(&self) -> &bool {
        &self.create_window
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

            let right_chunk = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(layout[1]);

            let right_top_block = Block::default().borders(Borders::NONE);
            let right_bottom_block = Block::default().borders(Borders::NONE);

            // Render the two blocks
            f.render_widget(right_top_block, right_chunk[0]);
            f.render_widget(right_bottom_block, right_chunk[1]);

            let upper_right_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Percentage(70),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(right_chunk[0]);

            let upper_right_right_paragraph = {
                let (status_text, status_color) = if *mp_struct.get_task_running() {
                    ("Task Running", Color::Green)
                } else {
                    ("Task Not Running", Color::Red)
                };

                let location_text = format!(
                    "   Task Location: {}",
                    mp_struct.get_task_execution_location()
                );

                let (wandb_text, wandb_color) = if *mp_struct.get_wandb() {
                    ("   WANDB: True", Color::Green)
                } else {
                    ("   WANDB: False", Color::Red)
                };

                Paragraph::new(Text::from(vec![
                    Line::from(""),
                    Line::from(Span::styled(status_text, Style::default().fg(status_color))),
                    Line::from(""),
                    Line::from(Span::styled(
                        location_text,
                        Style::default().fg(Color::White),
                    )),
                    Line::from(Span::styled(wandb_text, Style::default().fg(wandb_color))),
                ]))
                .alignment(Alignment::Left)
                .block(Block::default().borders(Borders::NONE).title("Info"))
            };
            f.render_widget(upper_right_right_paragraph, upper_right_chunk[2]);

            let upper_right_left_block = Block::default().borders(Borders::NONE);
            f.render_widget(upper_right_left_block, upper_right_chunk[1]);

            let upper_right_leck_chunk = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Percentage(40),
                    ]
                    .as_ref(),
                )
                .split(upper_right_chunk[1]);

            let task_name_paragraph = Paragraph::new(mp_struct.get_temp_name()).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title("Task Name"),
            );
            f.render_widget(task_name_paragraph, upper_right_leck_chunk[1]);

            let environment_name_paragraph = Paragraph::new(mp_struct.get_temp_env()).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title("Environment"),
            );
            f.render_widget(environment_name_paragraph, upper_right_leck_chunk[3]);

            // Right side (optional placeholder block)
            let right_block = Block::default().borders(Borders::ALL).title("Options");
            f.render_widget(right_block, layout[1]);

            if *mp_struct.get_create_window() {
                let popup_area = centered_rect(50, 50, f.size()); // 50% width, 20% height of terminal

                let popup_block = Block::default()
                    .title("Create Task")
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .style(Style::default().fg(Color::White).bg(Color::Black));

                // Render your popup content inside popup_area
                f.render_widget(Clear, popup_area);

                let popup_window = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(1),
                            Constraint::Length(3),
                            Constraint::Length(1),
                            Constraint::Length(3),
                            Constraint::Length(1),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(popup_area);

                let task_name = Paragraph::new(mp_struct.get_temp_name()).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::Rounded)
                        .title("Task-Name"),
                );
                f.render_widget(task_name, popup_window[1]);

                let env_name = Paragraph::new(mp_struct.get_temp_env()).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::Rounded)
                        .title("Environment-Name"),
                );
                f.render_widget(env_name, popup_window[3]);

                let dir_name = Paragraph::new(mp_struct.get_temp_dir()).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::Rounded)
                        .title("Directory"),
                );
                f.render_widget(dir_name, popup_window[5]);

                f.render_widget(popup_block, popup_area);
            }
        })
        .unwrap();
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
