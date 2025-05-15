use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use std::{error, io::Error, time::Duration};
pub enum Actions {
    Quit,
    None,
}

pub fn handle_key_input(timeout: Duration) -> Option<Actions> {
    if event::poll(timeout).unwrap_or(false) {
        // If there is an event, try to read it
        if let Ok(Event::Key(key)) = event::read() {
            match key.code {
                KeyCode::Char('q') => return Some(Actions::Quit), // Quit if 'q' is pressed
                _ => {} // Handle other keys if needed (e.g., return None for non-'q' keys)
            }
        } else {
            // If event::read fails, return None
            return None;
        }
    }
    Some(Actions::None)
}
