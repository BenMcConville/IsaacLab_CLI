use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent},
    execute,
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use ratatui::{
    Terminal,
    backend::{self, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    prelude::Backend,
    text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{
    fs::File,
    io::{self, Read, Write},
    panic,
    path::Path,
    ptr::null,
    time::{Duration, Instant},
};
use std::{
    process::{Command, exit},
    thread::JoinHandle,
};
// mod app;
pub mod events;
use events::{Actions, handle_key_input};
pub mod uis;
use uis::{Mainpage, render_page};
pub mod app;
use app::{App, State};
// use event::{Event, EventHandler};
//

fn main() {
    let mut app = App::new_app();

    // Stdout is the output of the termianl and if used io::stdout().flush() all entries in terminal
    // buffer are flushed into termianl for display. execture handles event calles and flushes
    execute!(io::stdout(), EnterAlternateScreen, DisableMouseCapture);
    enable_raw_mode();

    let backend = CrosstermBackend::new(io::stdout()); // Creates backend for abstract terminal communication
    // Includes methods like size, clear, cursor pos, ...
    let mut terminal = Terminal::new(backend);
    match terminal {
        Ok(mut term) => run_app(&mut term, &mut app),
        _ => eprintln!("Error init terminal..."),
    }

    disable_raw_mode();
    execute!(io::stdout(), LeaveAlternateScreen, EnableMouseCapture);

    println!("->{:?}", app.task_template_task());

    println!("Finished");
}

// Main app loop function using handle_key_input
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
    // Shared status between main thread and worker thread
    let status = Arc::new(Mutex::new(true));
    let mut thread_handle: Option<JoinHandle<()>> = None;

    let mut mp_struct = Mainpage::new();
    app.set_state(app::State::Main);
    while *app.get_state() == app::State::Main {
        if *mp_struct.get_create_window() {
            task_creating(&mut mp_struct, app);
        } else {
            task_browsing(&mut mp_struct, app);
        }

        // Render UI in a separate function
        render_page(terminal, &mut mp_struct);

        let mut done = status.lock().unwrap();

        if *done {
            // Update mp_struct with new data
            mp_struct.update_task_list(app.get_task_queue_names());

            if thread_handle.is_none() && !app.task_queue_is_empty() {
                if let Some(task) = app.pop_first_task() {
                    *done = false; // Reset status
                    match write_yaml(task.get_directory(), task.get_yaml()) {
                        Ok(_) => {
                            let command = "echo test >> text.txt; sleep 10";
                            let status_clone = Arc::clone(&status);

                            thread_handle = Some(thread::spawn(move || {
                                run_bash_command(command);
                                // println!("Here");
                                let mut done = status_clone.lock().unwrap();
                                *done = true;
                            }));
                        }
                        Err(e) => {
                            eprintln!("Failed to write YAML file: {:?}", e);
                        }
                    }
                }
            } else {
                // Thread is done, just clear the handle — no join
                thread_handle = None;
                // println!("Thread finished, ready for next process.");
            }
        }
    }

    if let Some(handle) = thread_handle.take() {
        match handle.join() {
            Ok(_) => {
                println!("Thread finished successfully.");
            }
            Err(e) => {
                eprintln!("Thread panicked: {:?}", e);
            }
        }
    }
}

fn task_browsing(mp_struct: &mut Mainpage, app: &mut App) {
    // Call handle_key_input with a timeout of 5 milliseconds
    match handle_key_input(Duration::from_micros(5000), false) {
        Some(Actions::Quit) => {
            app.set_state(app::State::Exit);
        }
        Some(Actions::Createtask) => {
            app.create_new_template_task();
            mp_struct.set_create_window(true);
        }
        Some(Actions::Moveup) => {
            mp_struct.decrease_selection();
            mp_struct.set_active_view(false);
        }
        Some(Actions::Movedown) => {
            mp_struct.increase_selection();
            mp_struct.set_active_view(false);
        }
        Some(Actions::Enter) => {
            mp_struct.set_active_view(true);
            let name = mp_struct.get_current_task_selection_name().to_string(); // clone String
            if let Some((env, dir)) = app.get_task_info(&name) {
                mp_struct.set_temp_name(name);
                mp_struct.set_temp_env(env);
                mp_struct.set_temp_dir(dir);
            }
        }
        Some(Actions::None) => {
            // Optionally handle the case where no key is pressed
            // and the timeout occurs
            // println!("No input detected within the timeout.");
        }
        None => {
            // Error reading the event, handle gracefully
            eprintln!("Error reading key input.");
        }
        _ => {
            eprint!(" Unidenfitied Enum");
        }
    }
}

fn task_creating(mp_struct: &mut Mainpage, app: &mut App) {
    match handle_key_input(Duration::from_micros(5000), true) {
        // First, handle the None case from the first match
        None => {
            eprintln!("Error reading key input.");
        }
        Some(action) => {
            // Handle the template task logic if we have a valid key input
            let template_task = app.read_template_task();
            match *template_task {
                Some(ref task) => {
                    mp_struct.update_temp_task(
                        task.get_task_name(),
                        task.get_environment(),
                        task.get_directory(),
                    );
                }
                _ => (),
            }

            // Now, handle different actions from the second match based on the key input
            match action {
                Actions::Quit => {
                    mp_struct.set_create_window(false);
                }
                Actions::Char(c) => {
                    if *mp_struct.get_yaml_mode() {
                        mp_struct.write_to_yaml_buffer(c);
                    } else {
                        app.write_to_buffer(c);
                    }
                }
                Actions::Delete => {
                    if *mp_struct.get_yaml_mode() {
                        mp_struct.backspace_yaml_buffer();
                    } else {
                        app.pop_last_elem_from_buffer();
                    }
                }
                Actions::Tab => mp_struct.toggle_update_yaml_selection(),
                Actions::Moveup => {
                    app.move_down_fsm();
                    if app.is_yaml_state() {
                        mp_struct.set_yaml_mode(true);
                    } else {
                        mp_struct.set_yaml_mode(false);
                    }
                }
                Actions::Right => {
                    mp_struct.increment_yaml_selection();
                }
                Actions::Left => {
                    mp_struct.decrement_yaml_selection();
                }

                Actions::Movedown => {
                    app.move_up_fsm();
                    if app.is_yaml_state() {
                        mp_struct.set_yaml_mode(true);
                    } else {
                        mp_struct.set_yaml_mode(false);
                    }
                }
                Actions::Enter => {
                    if *mp_struct.get_yaml_mode() {
                        // mp_struct.apply_selected_yaml_update();
                        mp_struct.write_buff_to_yaml();
                        mp_struct.toggle_update_yaml_selection()
                    } else {
                        match mp_struct.take_yaml() {
                            Some(yaml) => app.set_yaml(yaml),
                            None => {}
                        }
                        app.pass_template_to_task_list();
                        mp_struct.update_task_list(app.get_task_queue_names());
                        mp_struct.set_create_window(false);
                    }
                }
                Actions::None => {
                    // Optionally handle the case where no key is pressed
                    // and the timeout occurs
                    // println!("No input detected within the timeout.");
                }
                _ => {
                    eprint!(" Unidentified Enum");
                }
            }
        }
    }
}

fn run_bash_command(command: &str) {
    let status = Command::new("bash")
        .arg("-c")
        .arg(command)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Command failed: {}", command);
        exit(1); // Or handle failure appropriately
    }
}

// Write a generic type T to a YAML file
fn write_yaml<T>(file_path: &str, data: &T) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize, // The type T must be serializable
{
    // Serialize the data into a YAML string
    let yaml_string = serde_yaml::to_string(data)?;

    // Open the file in write mode. If the file doesn't exist, it will be created.
    // If it exists, its content will be truncated (overwritten).
    let path = Path::new(file_path);
    let mut file = File::create(path)?;

    // Write the YAML string to the file
    file.write_all(yaml_string.as_bytes())?;

    Ok(())
}
