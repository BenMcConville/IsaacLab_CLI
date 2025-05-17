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
// mod app;
pub mod events;
use events::{Actions, handle_key_input};
pub mod uis;
use uis::{Mainpage, render_main_page_ui};
pub mod app;
use app::{App, State};
// use event::{Event, EventHandler};
//
#[derive(Debug, Deserialize)]
struct Config {
    current_task: String,
    current_dir: String,
}

fn main() {
    // Example 1: Deserialize into a Config struct
    match read_yaml::<Config>("./src/config.yaml") {
        Ok(config) => {
            println!("Task: {}", config.current_task);
            println!("Directory: {}", config.current_dir);
        }
        Err(e) => eprintln!("Error reading YAML file: {}", e),
    }

    // // Example 2: Deserialize into a serde_yaml::Value
    // match read_yaml::<Value>("./src/config.yaml") {
    //     Ok(value) => {
    //         println!("YAML Content: {:#?}", value);
    //     }
    //     Err(e) => eprintln!("Error reading YAML file: {}", e),
    // }
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
    let mut mp_struct = Mainpage::new();
    app.set_state(app::State::Main);
    while *app.get_state() == app::State::Main {
        if *mp_struct.get_create_window() {
            task_creating(&mut mp_struct, app);
        } else {
            task_browsing(&mut mp_struct, app);
        }

        // Render UI in a separate function
        render_main_page_ui(terminal, &mp_struct);
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
                    app.write_to_buffer(c);
                }
                Actions::Delete => {
                    app.pop_last_elem();
                }
                Actions::Moveup => {
                    app.move_down_fsm();
                }
                Actions::Movedown => {
                    app.move_up_fsm();
                }
                Actions::Enter => {
                    mp_struct.set_active_view(true);
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
