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
use std::{io, panic};
use std::{
    io::Write,
    time::{Duration, Instant},
};

// mod app;
pub mod events;
use events::{Actions, handle_key_input};
pub mod uis;
use uis::render_main_page_ui;
// use app::App;
// use event::{Event, EventHandler};

fn main() {
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
