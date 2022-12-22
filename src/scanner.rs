mod err;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
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
        }
    }

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
            //TODO: Add string support
            _ => {
                err::error(self.line, "Unexpected character.");
                break;
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
}
