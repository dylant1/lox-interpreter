pub struct Token {
    //TODO: change token type 
    pub token_type: String,
    pub lexeme: String,
    //TODO: change literal type
    pub literal: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: String, lexeme: String, literal: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
