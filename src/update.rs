use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub fn update(app: &mut App, key_event: KeyEvent) {
    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            KeyCode::Esc => app.quit(),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.reset_current()
                }
            }
            KeyCode::Enter => {
                app.push_history(app.current_calc);
                app.reset_current();
            }
            KeyCode::Char('z') | KeyCode::Char('Z') => {
                app.push_current('0');
            }
            KeyCode::Char('o') | KeyCode::Char('O') => {
                app.push_current('1');
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                app.push_current('2');
            }
            KeyCode::Char('h') | KeyCode::Char('H') => {
                app.push_current('3');
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                app.push_current('4');
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                app.push_current('5');
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                app.push_current('6');
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                app.push_current('7');
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                app.push_current('8');
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                app.push_current('9');
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                app.push_current('+');
            }
            KeyCode::Char('u') | KeyCode::Char('U') => {
                app.push_current('-');
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                app.push_current('*');
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                app.push_current('/');
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                app.push_current('^');
            }
            _ => {}
        }
    };
}
