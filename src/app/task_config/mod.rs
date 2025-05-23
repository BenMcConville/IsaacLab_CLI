#[derive(Debug)]
pub struct Task {
    task_name: String,
    environment: String,
    directory: String,
}

impl Task {
    pub fn new() -> Self {
        Self {
            task_name: String::from(""),
            environment: String::from(""),
            directory: String::from(""),
        }
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
