use serde_yaml::{Value, from_str};

#[derive(Debug)]
pub struct Task {
    task_name: String,
    environment: String,
    directory: String,
    yaml: Option<Value>,
}

impl Task {
    pub fn new() -> Self {
        Self {
            task_name: String::from(""),
            environment: String::from(""),
            directory: String::from(""),
            yaml: None,
        }
    }
    pub fn get_yaml(&self) -> &Option<Value> {
        &self.yaml
    }
    pub fn set_yaml(&mut self, yaml: Value) {
        self.yaml = Some(yaml);
    }
    pub fn get_task_name(&self) -> &str {
        &self.task_name
    }
    pub fn get_environment(&self) -> &str {
        &self.environment
    }
    pub fn get_directory(&self) -> &str {
        &self.directory
    }
    pub fn set_task_name(&mut self, string: String) {
        self.task_name = string;
    }
    pub fn set_environment(&mut self, string: String) {
        self.environment = string;
    }
    pub fn set_directory(&mut self, string: String) {
        self.directory = string;
    }
}
