#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub history: Vec<String>,
    pub current_calc: String,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn reset_current(&self) {
        self.current_calc = String::new();
    }

    pub fn push_current(&self, s: char) {
        self.current_calc.push(s);
    }

    pub fn push_history(&self, s: String) {
        self.history.push(s);
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
