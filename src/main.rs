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
    io,
    io::{Read, Write},
    panic,
    path::Path,
    time::{Duration, Instant},
};
// mod app;
pub mod events;
use events::{Actions, handle_key_input};
pub mod uis;
use uis::render_main_page_ui;
pub mod app;
// use app::App;
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

    // Example 2: Deserialize into a serde_yaml::Value
    match read_yaml::<Value>("./src/config.yaml") {
        Ok(value) => {
            println!("YAML Content: {:#?}", value);
        }
        Err(e) => eprintln!("Error reading YAML file: {}", e),
    }

    // Stdout is the output of the termianl and if used io::stdout().flush() all entries in terminal
    // buffer are flushed into termianl for display. execture handles event calles and flushes
    execute!(io::stdout(), EnterAlternateScreen, DisableMouseCapture);
    enable_raw_mode();

    let backend = CrosstermBackend::new(io::stdout()); // Creates backend for abstract terminal communication
    // Includes methods like size, clear, cursor pos, ...
    let mut terminal = Terminal::new(backend);
    match terminal {
        Ok(mut term) => run_app(&mut term),
        _ => println!("Error init terminal..."),
    }

    disable_raw_mode();
    execute!(io::stdout(), LeaveAlternateScreen, EnableMouseCapture,);

    println!("Finished");
}

// Main app loop function using handle_key_input
fn run_app<B: Backend>(terminal: &mut Terminal<B>) {
    loop {
        // Call handle_key_input with a timeout of 5 milliseconds
        match handle_key_input(Duration::from_micros(5000)) {
            Some(Actions::Quit) => {
                println!("Quitting the app...");
                break; // Break the loop if 'q' is pressed
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
        }

        // Render UI in a separate function
        render_main_page_ui(terminal);
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
