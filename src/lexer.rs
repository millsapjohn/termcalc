pub enum TokenType {
    left_paren,
    right_paren,
    star,
    cross,
    slash,
    dash,
    period,
    caret,
    exclamation,
    keyword,
    variable,
    digit,
}

// #[derive(Debug, Default)]
pub struct Scanner {
    no_left_parens: i64,
    no_right_parens: i64,
    tokens: Vec<TokenType>,
    is_valid: bool,
    expression: String,
}

impl Struct {
    pub fn new(input: String) -> Self {
        Self {
            no_left_parens: 0,
            no_right_parens: 0,
            tokens: Vec::new(),
            is_valid: true,
            expression: input,
        };
    }

    pub fn increment_left(&mut self) {
        self.no_left_parens += 1;
    }

    pub fn increment_right(&mut self) {
        self.no_right_parens += 1;
    }

    pub fn invalidate(&mut self) {
        self.is_valid = false;
    }

    pub fn push_token(&mut self, c: TokenType) {
        self.tokens.push(c)
    }

    pub fn scan(&mut self) {
        for c in self.expression {
            match c {
                '/' => self.push_token(TokenType::slash),
                '+' => self.push_token(TokenType::cross),
                '-' => self.push_token(TokenType::dash),
                '*' => self.push_token(TokenType::star),
                '^' => self.push_token(TokenType::caret),
                '!' => self.push_token(TokenType::exclamation),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    self.push_token(TokenType::digit)
                }
                '.' => self.push_token(TokenType::period),
                '(' => {
                    self.push_token(TokenType::left_paren);
                    self.increment_left;
                }
                ')' => {
                    self.push_token(TokenType::right_paren);
                    self.increment_right;
                }
                _ => {}
            }
        }
    }
}
