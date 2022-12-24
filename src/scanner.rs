use std::collections::HashMap;
mod err;

//create a keyword map
fn keyword_map() -> HashMap<&'static str, &'static str> {
    //insert tokentypes into hashmap

    let mut map = HashMap::new();
    map.insert("and", TokenType::AND);
    map.insert("class", TokenType::CLASS);
    map.insert("else", TokenType::ELSE);
    map.insert("false", TokenType::FALSE);
    map.insert("for", TokenType::FOR);
    map.insert("fun", TokenType::FUN);
    map.insert("if", TokenType::IF);
    map.insert("nil", TokenType::NIL);
    map.insert("or", TokenType::OR);
    map.insert("print", TokenType::PRINT);
    map.insert("return", TokenType::RETURN);
    map.insert("super", TokenType::SUPER);
    map.insert("this", TokenType::THIS);
    map.insert("true", TokenType::TRUE);
    map.insert("var", TokenType::VAR);
    map.insert("while", TokenType::WHILE);
    map
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, &'static str>,
}

impl Scanner {
    //TODO: May need to change the start, current, and line props
    fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: keyword_map(),
        }
    }
    //inserts keywords into the hashmap


    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "", None, self.line));
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token() {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '>' => {
                self.add_token(
                    if self.match_char('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    },
                    None,
                )
            },
            '<' => {
                self.add_token(
                    if self.match_char('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    },
                    None,
                )
            },
            '=' => {
                self.add_token(
                    if self.match_char('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    },
                    None,
                )
            },
            '!' => {
                self.add_token(
                    if self.match_char('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    },
                    None,
                )
            },
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    add_token(TokenType::Slash, None);
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    err::error(self.line, "Unexpected character.");
                }
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self.keywords.get(text).unwrap_or(&TokenType::IDENTIFIER);
        self.add_token(token_type, None);
    }


    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    fn string() {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            err::error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Some(Literal::String(value)));
    }

    fn is_digit(c: char) -> bool {
        //TODO: Check if this is correct
        c >= '0' && c <= '9'
    }

    fn number() {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == "." && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        //TODO: clean this up
        self.add_token(TokenType::Number, Some(Literal::Number(self.source[self.start..self.current].parse::<f64>().unwrap())));
    }

    fn is_alpha(c: char) -> bool {
        //TODO: Check if this is correct
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

}
