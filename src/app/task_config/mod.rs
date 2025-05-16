pub struct Task {
    task_name: String,
}

impl Task {
    pub fn new(task_name: String) -> Self {
        Self { task_name }
    }
    pub fn get_name(&self) -> &str {
        &self.task_name
    }
}
