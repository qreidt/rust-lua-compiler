use std::fmt;

pub struct Lexer {
    source: Vec<char>,
    pub current_char: char,
    pub current_position: i32,
}

impl Lexer {
    pub fn new(source: Vec<char>) -> Self {

        let mut lexer = Lexer {
            source: source,
            current_char: '\n',
            current_position: -1
        };

        
        lexer.source.push('\n');
        lexer.next_char(1);

        return lexer;
    }

    pub fn next_char(&mut self, steps: i32) -> () {
        self.current_position += steps;
        if self.current_position >= self.source.len() as i32 {
            self.current_char = '\0';
            return;
        }

        self.current_char = *self.source.get(self.current_position as usize).unwrap();
    }

    pub fn peek(&self) -> char {
        if ((self.current_position + 1) as usize) >= self.source.len() {
            return '\0';
        }

        return *self.source.get((self.current_position + 1) as usize).unwrap()
    }

    pub fn get_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();

        let combination = [self.current_char, self.peek()];
        let token: Token = match combination {
            ['+',  _ ] => Token::new(self.current_char.to_string(), TokenType::PLUS),
            ['-',  _ ] => Token::new(self.current_char.to_string(), TokenType::MINUS),
            ['*',  _ ] => Token::new(self.current_char.to_string(), TokenType::ASTERISK),
            ['/',  _ ] => Token::new(self.current_char.to_string(), TokenType::SLASH),
            ['\n', _ ] => Token::new(self.current_char.to_string(), TokenType::NEWLINE),

            ['>', '='] => Token::new(String::from_iter(combination), TokenType::GTEQ),
            ['>',  _ ] => Token::new(self.current_char.to_string(), TokenType::GT),

            ['<', '='] => Token::new(String::from_iter(combination), TokenType::LTEQ),
            ['<',  _ ] => Token::new(self.current_char.to_string(), TokenType::LT),

            ['!', '='] => Token::new(String::from_iter(combination), TokenType::NOTEQ),
            ['!',  _ ] => Token::new(self.current_char.to_string(), TokenType::NOT),

            ['=', '='] => Token::new(String::from_iter(combination), TokenType::EQEQ),
            ['=',  _ ] => Token::new(self.current_char.to_string(), TokenType::EQ),
            
            ['\0', _ ] => Token::new(self.current_char.to_string(), TokenType::EOF),
            _ => self.check_unknown_char()
        };

        // abort if Unknown 
        if token.ttype == TokenType::Unknown {
            let mut message = "Unknown token! ".to_string();
            message.push(self.current_char);
            self.abort(message);
        }

        if [TokenType::STRING, TokenType::NUMBER].contains(&token.ttype) {
            self.next_char(1);
            return token;
        }

        // Jump to next char if using double token
        if [TokenType::GTEQ, TokenType::LTEQ, TokenType::EQEQ, TokenType::NOTEQ].contains(&token.ttype) {
            self.next_char(2);
            return token;
        }

        self.next_char(1);
        return token;

    }

    fn skip_whitespace(&mut self) {
        while [' ', '\t', '\r'].contains(&self.current_char) {
            self.next_char(1);
        }
    }

    fn skip_comment(&mut self) -> () {
        if self.current_char == '-' && self.peek() == '-' {
            while self.current_char != '\n' {
                self.next_char(1);
            }
        }
    }

    fn check_unknown_char(&mut self) -> Token {

        // Try to get string token
        let string_token_option = self.check_string();
        if let Some(string_token) = string_token_option {
            return string_token;
        }

        // Try to get numeric token
        let numeric_token_option = self.check_numeric();
        if let Some(numeric_token) = numeric_token_option {
            return numeric_token;
        }

        // Try to get ident token
        let ident_token_option = self.check_ident();
        if let Some(ident_token) = ident_token_option {
            return ident_token;
        }

        // No token found
        return Token::new(self.current_char.to_string(), TokenType::Unknown);
    }

    fn check_string(&mut self) -> Option<Token> {
        if self.current_char == '\"' {
            self.next_char(1);
            let start_pos = self.current_position as usize;

            while self.current_char != '\"' {
                if ['\r', '\n', '\t', '\\', '%'].contains(&self.current_char) {
                    self.abort("Illegal character in string".to_string());
                }

                self.next_char(1);
            }

            let end_pos = self.current_position as usize;
            let text_token = String::from_iter(&self.source[start_pos..end_pos]);
            return Some(Token::new(text_token, TokenType::STRING));
        }

        return None;
    }

    fn check_numeric(&mut self) -> Option<Token> {
        if ! self.current_char.is_digit(10) {
            return None;
        }
        
        let start_pos = self.current_position as usize;

        // read while 123...
        while self.peek().is_digit(10) {
            self.next_char(1);
        }

        // if 123. ...
        if self.peek() == '.' {

            self.next_char(1);

            // end if 123. 1as0q83h
            if ! self.peek().is_digit(10) {
                self.abort("Illegal character in number.".to_string());
            }

            // read while 123.123...
            while self.peek().is_digit(10) {
                self.next_char(1);
            }
        }

        let end_pos = (self.current_position + 1) as usize;
        let token_text = String::from_iter(&self.source[start_pos..end_pos]);

        return Some(Token::new(token_text, TokenType::NUMBER));
        
    }

    fn check_ident(&mut self) -> Option<Token> {
        // Leading character is a letter, so this must be an identifier or a keyword.
        if ! self.current_char.is_alphabetic() {
            return None;
        }

        // Get all consecutive alpha numeric characters.
        let start_pos = self.current_position as usize;
        while self.peek().is_alphanumeric() {
            self.next_char(1);
        }

        // Check if the token is in the list of keywords.
        let end_pos = (self.current_position + 1) as usize;
        let token_text = String::from_iter(&self.source[start_pos..end_pos]);
        let ident_option = match TokenType::try_from(token_text.to_uppercase().as_str()) {
            Ok(ttype) => Some(ttype),
            Err(_) => None,
        };

        // Check if the identifier found is valid
        if let Some(ident) = ident_option {
            let type_value = ident.clone() as i32;
            if type_value >= 100 && type_value < 200 {
                return Some(Token::new(token_text, ident))
            }
        }

        // Return new identifier
        return Some(Token::new(token_text, TokenType::IDENT));
    }

    fn abort(&self, message: String) -> () {
        println!("Aborted! {}", message);
        std::process::exit(1)
    }
}

#[derive(Clone)]
pub struct Token {
    pub text: String,
    pub ttype: TokenType
}

impl Token {
    pub fn new(text: String, ttype: TokenType) -> Self {
        return Token{text, ttype};
    }
}

use strum_macros::EnumString;

#[derive(PartialEq)]
#[derive(EnumString)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum TokenType {
    Unknown = -2,
    EOF = -1,
	NEWLINE = 0,
	NUMBER = 1,
	IDENT = 2,
	STRING = 3,
	
    // Keywords.
	LABEL = 101,
	GOTO = 102,
	PRINT = 103,
	INPUT = 104,
	LET = 105,
	IF = 106,
	THEN = 107,
	ENDIF = 108,
	WHILE = 109,
	REPEAT = 110,
	ENDWHILE = 111,
	
    // Operators.
	EQ = 201,
    NOT = 202,
	PLUS = 203,
	MINUS = 204,
	ASTERISK = 205,
    SLASH = 206,
	EQEQ = 207,
	NOTEQ = 208,
	LT = 209,
	LTEQ = 210,
	GT = 211,
	GTEQ = 212,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}
