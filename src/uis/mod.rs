pub mod main_page;
use main_page::render_main_page_ui;
pub mod yaml_page;
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
use yaml_page::render_yaml_page_ui;

pub struct Mainpage {
    task_selection: usize,
    active_view: bool,
    task_list: Vec<String>,
    task_running: bool,
    task_execution_location: String,
    wandb: bool,
    create_window: bool,
    yaml_mode: bool,
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
            yaml_mode: false,
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
    pub fn get_yaml_mode(&self) -> &bool {
        &self.yaml_mode
    }
    pub fn set_yaml_mode(&mut self, yaml_mode: bool) {
        self.yaml_mode = yaml_mode;
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

pub fn render_page<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, mp_struct: &Mainpage) {
    if *mp_struct.get_yaml_mode() {
        render_yaml_page_ui(terminal, mp_struct);
    } else {
        render_main_page_ui(terminal, mp_struct);
    }
}
