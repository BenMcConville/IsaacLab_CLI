use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use std::{error, io::Error, time::Duration};
pub enum Actions {
    Quit,
    Enter,
    Char(char),
    Createtask,
    Moveup,
    Movedown,
    None,
    Delete,
}

pub fn handle_key_input(timeout: Duration, read_key_stroke: bool) -> Option<Actions> {
    if read_key_stroke {
        return read_key_strokes(timeout);
    } else {
        return traverse_with_keys(timeout);
    }
}
pub fn traverse_with_keys(timeout: Duration) -> Option<Actions> {
    if event::poll(timeout).unwrap_or(false) {
        // If there is an event, try to read it
        if let Ok(Event::Key(key)) = event::read() {
            match key.code {
                KeyCode::Esc => return Some(Actions::Quit), // Quit if 'q' is pressed
                KeyCode::Char('c') => return Some(Actions::Createtask), // Quit if 'q' is pressed
                KeyCode::Up => return Some(Actions::Moveup),
                KeyCode::Down => return Some(Actions::Movedown),
                KeyCode::Enter => return Some(Actions::Enter),
                _ => {} // Handle other keys if needed (e.g., return None for non-'q' keys)
            }
        } else {
            // If event::read fails, return None
            return None;
        }
    }
    Some(Actions::None)
}

pub fn read_key_strokes(timeout: Duration) -> Option<Actions> {
    if event::poll(timeout).unwrap_or(false) {
        // If there is an event, try to read it
        if let Ok(Event::Key(key)) = event::read() {
            match key.code {
                KeyCode::Char(c) => return Some(Actions::Char(c)),
                KeyCode::Up => return Some(Actions::Moveup),
                KeyCode::Down => return Some(Actions::Movedown),
                KeyCode::Enter => return Some(Actions::Enter),
                KeyCode::Esc => return Some(Actions::Quit),
                KeyCode::Backspace => return Some(Actions::Delete),
                _ => {} // Handle other keys if needed (e.g., return None for non-'q' keys)
            }
        } else {
            // If event::read fails, return None
            return None;
        }
    }
    Some(Actions::None)
}
