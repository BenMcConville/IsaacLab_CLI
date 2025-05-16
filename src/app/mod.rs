pub mod task_config;
use task_config::Task;

enum State {
    Enter,
    Main,
    Exit,
}

pub struct App {
    state: State,
    task_queue: Vec<Task>,
}

impl App {
    pub fn new_app() -> Self {
        Self {
            state: State::Enter,
            task_queue: vec![],
        }
    }
    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn get_task_queue_names(&self) -> Vec<&str> {
        let mut task_queue_names: Vec<&str> = vec![];
        for task in &self.task_queue {
            task_queue_names.push(task.get_name());
        }
        task_queue_names
    }
}
