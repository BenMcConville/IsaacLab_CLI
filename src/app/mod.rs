pub mod task_config;
use task_config::Task;
#[derive(PartialEq, Debug)]
pub enum State {
    Enter,
    Main,
    Exit,
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CreationState {
    Taskname,
    Envname,
    Dir,
    Null,
}

pub struct App {
    state: State,
    creation_state: CreationState,
    task_queue: Vec<Task>,
    template_task: Option<Task>, // Task being created before added to queue
}

impl App {
    pub fn new_app() -> Self {
        Self {
            state: State::Enter,
            creation_state: CreationState::Null,
            task_queue: vec![],
            template_task: None,
        }
    }
    pub fn get_state(&self) -> &State {
        &self.state
    }
    pub fn get_creation_state(&self) -> &CreationState {
        &self.creation_state
    }
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
    pub fn set_creation_state(&mut self, state: CreationState) {
        self.creation_state = state;
    }
    pub fn create_new_template_task(&mut self) {
        self.creation_state = CreationState::Taskname;
        let temp_task = Task::new();
        self.template_task = Some(temp_task);
    }

    pub fn get_task_queue_names(&self) -> Vec<&str> {
        let mut task_queue_names: Vec<&str> = vec![];
        for task in &self.task_queue {
            task_queue_names.push(task.get_task_name());
        }
        task_queue_names
    }

    pub fn add_task_to_queue(&mut self, task: Task) {
        self.task_queue.push(task);
    }

    pub fn return_template_task(&mut self) -> Option<Task> {
        self.template_task.take()
    }

    pub fn write_to_buffer(&mut self, c: char) {
        // Copy out the enum (no borrow of self)!
        let creation_state = *self.get_creation_state();

        if let Some(task) = self.template_task.as_mut() {
            if creation_state == CreationState::Taskname {
                let mut name = task.get_task_name().to_string();
                name.push(c);
                task.set_task_name(name);
            }
        }
    }
    pub fn pop_last_elem(&mut self) {
        // Copy out the enum (no borrow of self)!
        let creation_state = *self.get_creation_state();

        if let Some(task) = self.template_task.as_mut() {
            if creation_state == CreationState::Taskname {
                let mut name = task.get_task_name().to_string();
                name.pop();
                task.set_task_name(name);
            }
        }
    }
}
