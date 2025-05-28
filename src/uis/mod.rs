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
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use serde_yaml::{Number, Value};
use std::collections::HashMap;
use std::str::FromStr;
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
            yaml_update_text: String::from(""),
        }
    }
    pub fn take_yaml(&mut self) -> Option<Value> {
        self.temp_yaml.take()
    }

    // ------------ Update yaml ----------------
    pub fn write_buff_to_yaml(&mut self) {
        match &mut self.temp_yaml {
            Some(yaml) => {
                // 'yaml' is &mut Value
                // Get the flattened key based on the current selection.
                // get_flattened_key_by_index takes &Value, which `yaml` already is.
                let key = get_flattened_key_by_index(yaml, self.yaml_selection);

                match key {
                    Some(key_str) => {
                        // The `set_nested_value_mut` function takes ownership of the String for the new value.
                        // So, we need to clone `self.yaml_update_text` to pass it.
                        let result =
                            set_nested_value_mut(yaml, &key_str, self.yaml_update_text.clone());

                        // Handle the result of the update operation
                        match result {
                            Ok(_) => {
                                // println!("Successfully updated key: {}", key_str);
                                // You might want to clear `yaml_update_text` here or reset `update_yaml_selection`
                                // self.yaml_update_text.clear();
                                // self.update_yaml_selection = false;
                            }
                            Err(e) => {
                                eprintln!("Error updating YAML for key {}: {}", key_str, e);
                            }
                        }
                    }
                    None => {
                        println!("No key found at index {}.", self.yaml_selection);
                    }
                }
            }
            None => {
                println!("Cannot update YAML: temp_yaml is not loaded.");
            }
        }
        self.yaml_update_text = String::from("");
    }

    // -------------------------------------------

    /// Helper to get the number of displayable lines from the YAML.
    /// This is equivalent to the `yaml_to_lines` helper from the previous response's example.
    // fn get_yaml_line_count(&self) -> usize {
    //     if let Some(yaml_value) = &self.temp_yaml {
    //         // Convert to string and count lines
    //         serde_yaml::to_string(yaml_value)
    //             .unwrap_or_else(|_| "".to_string()) // Handle error gracefully by returning empty string
    //             .lines()
    //             .count()
    //     } else {
    //         0 // No YAML data means 0 lines
    //     }
    // }
    fn get_yaml_line_count(&self) -> usize {
        if let Some(yaml_value) = &self.temp_yaml {
            match yaml_value {
                Value::Mapping(root_map) => {
                    // If the root is a map, we iterate through its *values*.
                    // We DO NOT count the `root_map.len()` here.
                    // Instead, we sum the counts from what each value contains.
                    let mut total_nested_count = 0;
                    for (_key, nested_value) in root_map {
                        // Call the helper to count all key-value pairs found within this nested value.
                        total_nested_count += count_all_found_key_value_pairs(nested_value);
                    }
                    total_nested_count
                }
                _ => {
                    // If the root is not a map (e.g., a sequence or a scalar),
                    // there are no "nested" key-value pairs of the type you're asking for.
                    // For example, if the YAML was just "foo: bar", "foo" would be top-level.
                    0
                }
            }
        } else {
            0 // No YAML data means 0 key-value pairs
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
fn flatten_value(value: &Value) -> Vec<(String, Value)> {
    let mut flat_vec = Vec::new(); // Changed to Vec
    flatten_recursive(value, String::new(), &mut flat_vec);
    flat_vec
}

// Recursive helper function to traverse the Value and build the flattened vector
// Now takes `&mut Vec<(String, Value)>` for `flat_vec`
fn flatten_recursive(value: &Value, prefix: String, flat_vec: &mut Vec<(String, Value)>) {
    if let Some(map) = value.as_mapping() {
        for (key_val, val_ref) in map {
            let key_str = if let Some(s) = key_val.as_str() {
                s.to_string()
            } else {
                serde_yaml::to_string(key_val)
                    .expect("Failed to serialize YAML key to string for flattening")
                    .trim_end_matches('\n')
                    .to_string()
            };

            let new_prefix = if prefix.is_empty() {
                key_str
            } else {
                format!("{}.{}", prefix, key_str)
            };
            flatten_recursive(val_ref, new_prefix, flat_vec); // Pass the vec reference
        }
    } else if let Some(arr) = value.as_sequence() {
        flat_vec.push((prefix, Value::Sequence(arr.clone()))); // Push to vec
    } else {
        flat_vec.push((prefix, value.clone())); // Push to vec
    }
}

/// Flattens a `serde_yaml::Value` and returns the key at the specified index
/// from the original-order flattened key-value pairs.
///
/// Takes a reference to `serde_yaml::Value`.
/// Returns `None` if the index is out of bounds or the Value cannot be flattened
/// into key-value pairs.
fn get_flattened_key_by_index(yaml_value: &Value, index: usize) -> Option<String> {
    // 1. Flatten the Value into a Vec, preserving order
    let kv_pairs = flatten_value(yaml_value); // No longer a HashMap, directly a Vec

    // 2. The vector is already in the order they appeared, so NO SORTING NEEDED
    // kv_pairs.sort_by(|(key1, _), (key2, _)| key1.cmp(key2)); // <--- REMOVE THIS LINE!

    // 3. Try to get the key at the specified index
    kv_pairs.get(index).map(|(key, _)| key.clone())
}
fn set_nested_value_mut(
    root: &mut Value,
    flattened_key: &str,
    new_string_value: String,
) -> Result<(), String> {
    let path_segments: Vec<&str> = flattened_key.split('.').collect();

    // --- New logic to parse new_string_value ---
    let parsed_value = if let Ok(int_val) = new_string_value.parse::<i64>() {
        Value::Number(Number::from(int_val))
    } else if let Ok(float_val) = new_string_value.parse::<f64>() {
        Value::Number(Number::from_str(&new_string_value).unwrap()) // Or handle error if FromStr fails (unlikely here)
    } else if new_string_value.eq_ignore_ascii_case("true") {
        Value::Bool(true)
    } else if new_string_value.eq_ignore_ascii_case("false") {
        Value::Bool(false)
    } else {
        Value::String(new_string_value)
    };
    // --- End of new logic ---

    if path_segments.is_empty() {
        // If the flattened key is empty, it means we are trying to replace the root.
        *root = parsed_value; // Use parsed_value here
        return Ok(());
    }

    let mut current_val = root;
    let last_segment_index = path_segments.len() - 1;

    for (i, &segment) in path_segments.iter().enumerate() {
        // Convert segment to a Value for use as a key in serde_yaml::Mapping
        let segment_key_val = Value::String(segment.to_string());

        if i == last_segment_index {
            // This is the final segment, so update the value with the parsed value
            let map = current_val
                .as_mapping_mut()
                .ok_or_else(|| format!("Parent of final key '{}' is not a mapping", segment))?;
            map.insert(segment_key_val, parsed_value); // <--- Use parsed_value here
            return Ok(());
        } else {
            // This is an intermediate segment, navigate deeper.
            let map = current_val
                .as_mapping_mut()
                .ok_or_else(|| format!("Path segment '{}' is not a mapping", segment))?;

            // Get or insert a new mapping for the next level
            current_val = map
                .entry(segment_key_val)
                .or_insert_with(|| Value::Mapping(serde_yaml::Mapping::new()));
        }
    }
    Ok(()) // Should be unreachable if path_segments is not empty
}

fn count_all_found_key_value_pairs(value: &Value) -> usize {
    match value {
        Value::Mapping(map) => {
            // For any map encountered (including nested ones),
            // count its direct key-value pairs AND recurse into its values.
            let mut count = map.len(); // Count direct key-value pairs in this map
            for (_key, val) in map {
                count += count_all_found_key_value_pairs(val); // Add counts from nested values
            }
            count
        }
        Value::Sequence(sequence) => {
            // For a sequence (list), iterate through its elements and
            // recursively count key-value pairs within each element.
            let mut count = 0;
            for item in sequence {
                count += count_all_found_key_value_pairs(item);
            }
            count
        }
        _ => {
            // Scalars (strings, numbers, booleans, null) don't have key-value pairs,
            // so they contribute 0 to the count.
            0
        }
    }
}
