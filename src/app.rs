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

    pub fn reset_current(&mut self) {
        self.current_calc = String::new();
    }

    pub fn push_current(&mut self, s: char) {
        self.current_calc.push(s);
    }

    pub fn push_history(&mut self) {
        self.history.push(self.current_calc.to_string());
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
