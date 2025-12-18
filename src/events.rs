use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_event(key: KeyEvent, app: &mut App) {
    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
        app.should_quit = true;
        return;
    }

    match key.code {
        KeyCode::Char('h') | KeyCode::Left => app.on_left(),
        KeyCode::Char('l') | KeyCode::Right => app.on_right(),
        KeyCode::Char('j') | KeyCode::Down => app.on_down(),
        KeyCode::Char('k') | KeyCode::Up => app.on_up(),

        KeyCode::Enter => {
            // to be made
        }
        _ => {}
    }
}
