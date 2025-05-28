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
use serde::Deserialize;
use serde_yaml::{Value, from_str};
use std::{
    fs::File,
    io::{self, Read, Write},
    panic,
    path::Path,
    ptr::null,
    time::{Duration, Instant},
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
    temp_yaml: Option<Value>,
    yaml_selection: usize,
    update_yaml_selection: bool,
    yaml_update_text: String,
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
            temp_yaml: None,
            yaml_selection: 0,
            update_yaml_selection: false,
            yaml_update_text: String::from("Test Text"),
        }
    }
    /// Helper to get the number of displayable lines from the YAML.
    /// This is equivalent to the `yaml_to_lines` helper from the previous response's example.
    fn get_yaml_line_count(&self) -> usize {
        if let Some(yaml_value) = &self.temp_yaml {
            // Convert to string and count lines
            serde_yaml::to_string(yaml_value)
                .unwrap_or_else(|_| "".to_string()) // Handle error gracefully by returning empty string
                .lines()
                .count()
        } else {
            0 // No YAML data means 0 lines
        }
    }
    pub fn write_to_yaml_buffer(&mut self, c: char) {
        // Append the character to the yaml_update_text string
        self.yaml_update_text.push(c);
    }

    pub fn backspace_yaml_buffer(&mut self) {
        // Remove the last character from the yaml_update_text string, if it's not empty
        self.yaml_update_text.pop();
    }
    pub fn toggle_update_yaml_selection(&mut self) {
        self.update_yaml_selection = !self.update_yaml_selection;
    }

    /// Increments `yaml_selection`, wrapping around to the beginning if it
    /// exceeds the total number of YAML lines.
    pub fn increment_yaml_selection(&mut self) {
        let line_count = self.get_yaml_line_count();
        if line_count == 0 {
            self.yaml_selection = 0; // No lines to select, so keep at 0
            return;
        }

        self.yaml_selection = (self.yaml_selection + 1) % line_count;
    }

    /// Decrements `yaml_selection`, wrapping around to the end if it
    /// goes below 0.
    pub fn decrement_yaml_selection(&mut self) {
        let line_count = self.get_yaml_line_count();
        if line_count == 0 {
            self.yaml_selection = 0; // No lines to select, so keep at 0
            return;
        }

        // Handle underflow by wrapping around to the last item
        self.yaml_selection = (self.yaml_selection + line_count - 1) % line_count;
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
    pub fn set_yaml_file(&mut self, yaml_file: Value) {
        self.temp_yaml = Some(yaml_file);
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

pub fn render_page<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mp_struct: &mut Mainpage,
) {
    if *mp_struct.get_yaml_mode() {
        render_yaml_page_ui(terminal, mp_struct);
    } else {
        render_main_page_ui(terminal, mp_struct);
    }

    match mp_struct.temp_yaml {
        None => {
            match read_yaml::<Value>("/Users/bmcc/Desktop/Test/config.yaml") {
                Ok(value) => mp_struct.set_yaml_file(value),
                Err(e) => (),
            };
        }
        _ => (),
    }
    // Example 1: Deserialize into a Config struct
    // match read_yaml::<Config>("/Users/bmcc/Desktop/Test/config.yaml") {
    //     Ok(config) => {
    //         println!("Task: {}", config.current_task);
    //         println!("Directory: {}", config.current_dir);
    //     }
    //     Err(e) => eprintln!("Error reading YAML file: {}", e),
    // }

    // // Example 2: Deserialize into a serde_yaml::Value
    // let mut yaml_data = match read_yaml::<Value>("/Users/bmcc/Desktop/Test/config.yaml") {
    //     Ok(value) => Some(value),
    //     Err(e) => None,
    // };

    // let mut add_status_updater = |v: &mut Value| {
    //     if let Value::Mapping(map) = v {
    //         if map.contains_key("Task") {
    //             // Changed from "config"
    //             map.insert(
    //                 Value::String("Task".to_string()),
    //                 Value::String("active".to_string()),
    //             );
    //         }
    //     }
    // };
    // match yaml_data {
    //     Some(mut data) => {
    //         update_yaml_elements(&mut data, &mut add_status_updater);
    //         println!(
    //             "\nYAML after adding status field:\n{}",
    //             serde_yaml::to_string(&data).unwrap()
    //         );
    //     }
    //     None => eprintln!("Error reading YAML file"),
    // }
}

pub fn update_yaml_elements<F>(value: &mut Value, updater: &mut F)
where
    F: FnMut(&mut Value),
{
    match value {
        Value::Mapping(map) => {
            // Apply updater to each value in the map
            for (_, v) in map.iter_mut() {
                updater(v);
                // Recursively call for nested values
                update_yaml_elements(v, updater);
            }
        }
        Value::Sequence(seq) => {
            // Apply updater to each item in the sequence
            for item in seq.iter_mut() {
                updater(item);
                // Recursively call for nested items
                update_yaml_elements(item, updater);
            }
        }
        // For other types (String, Number, Bool, Null), apply updater directly.
        // These types do not contain further nested Value types, so no recursion is needed.
        _ => {
            updater(value);
        }
    }
}

// Read YAML file into a generic type T
fn read_yaml<T>(file_path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>, // Deserializes for any lifetime
{
    // Open the YAML file
    let path = Path::new(file_path);
    let mut file = File::open(path)?;

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the YAML string into the specified type T
    let result: T = from_str(&contents)?; // contents is a String, so it lives long enough

    Ok(result)
}
