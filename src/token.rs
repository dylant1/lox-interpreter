//use tokentype
use crate::token_type::TokenType;
#[derive(Clone, Debug)]
pub struct Token {
    //TODO: change token type 
    pub token_type: TokenType,
    pub lexeme: String,
    //TODO: change literal type
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        //print out the token type, lexeme, and literal
        //but the token type is a enum
        println!(" {} ", self.lexeme);
        String::from("Token")
    }
}
