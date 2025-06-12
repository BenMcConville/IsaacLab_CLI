pub mod task_config;
use serde_yaml::{Value, from_str};
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
    Yaml,
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
    pub fn task_queue_is_empty(&self) -> bool {
        self.task_queue.is_empty()
    }
    pub fn pop_first_task(&mut self) -> Option<Task> {
        if !self.task_queue.is_empty() {
            Some(self.task_queue.remove(0))
        } else {
            None
        }
    }
    pub fn set_yaml(&mut self, yaml: Value) {
        if let Some(ref mut task) = self.template_task {
            task.set_yaml(yaml);
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
    pub fn get_task_info(&self, task_name: &str) -> Option<(&str, &str)> {
        for task in &self.task_queue {
            if task.get_task_name() == task_name {
                return Some((task.get_directory(), task.get_environment()));
            }
        }
        None
    }

    pub fn pass_template_to_task_list(&mut self) {
        match self.template_task.take() {
            Some(task) => self.add_task_to_queue(task),
            None => (),
        }
        self.template_task = None;
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

    pub fn task_template_task(&mut self) -> Option<Task> {
        self.template_task.take()
    }
    pub fn read_template_task(&self) -> &Option<Task> {
        &self.template_task
    }

    pub fn write_to_buffer(&mut self, c: char) {
        // Copy out the enum (no borrow of self)!
        let creation_state = *self.get_creation_state();

        if let Some(task) = self.template_task.as_mut() {
            if creation_state == CreationState::Taskname {
                let mut name = task.get_task_name().to_string();
                name.push(c);
                task.set_task_name(name);
            } else if creation_state == CreationState::Envname {
                let mut env_name = task.get_environment().to_string();
                env_name.push(c);
                task.set_environment(env_name);
            } else if creation_state == CreationState::Dir {
                let mut directory = task.get_directory().to_string();
                directory.push(c);
                task.set_directory(directory);
            }
        }
    }
    pub fn move_down_fsm(&mut self) {
        match self.creation_state {
            CreationState::Taskname => self.creation_state = CreationState::Envname,
            CreationState::Envname => self.creation_state = CreationState::Dir,
            CreationState::Dir => self.creation_state = CreationState::Yaml,
            CreationState::Yaml => self.creation_state = CreationState::Taskname,
            _ => (),
        }
    }
    pub fn is_yaml_state(&self) -> bool {
        self.creation_state == CreationState::Yaml
    }
    pub fn move_up_fsm(&mut self) {
        // Update depending on res
        match self.creation_state {
            CreationState::Taskname => self.creation_state = CreationState::Envname,
            CreationState::Envname => self.creation_state = CreationState::Dir,
            CreationState::Dir => self.creation_state = CreationState::Yaml,
            CreationState::Yaml => self.creation_state = CreationState::Taskname,
            _ => (),
        }
    }

    pub fn pop_last_elem_from_buffer(&mut self) {
        // Used for backspace <----------
        // Copy out the enum (no borrow of self)!
        let creation_state = *self.get_creation_state();

        if let Some(task) = self.template_task.as_mut() {
            if creation_state == CreationState::Taskname {
                let mut name = task.get_task_name().to_string();
                name.pop();
                task.set_task_name(name);
            }
            if creation_state == CreationState::Envname {
                let mut env = task.get_environment().to_string();
                env.pop();
                task.set_environment(env);
            }
            if creation_state == CreationState::Dir {
                let mut dir = task.get_directory().to_string();
                dir.pop();
                task.set_directory(dir);
            }
        }
    }
}
