mod lexer;
use lexer::{Lexer, TokenType};
// use std::fs;

// fn main() {

//     let contents = fs::read_to_string("./fibonacci.lua")
//         .expect("Should have been able to read the file");

//     let mut lexer = Lexer::new(contents.chars().collect());

//     while lexer.peek() != '\0' {
//         println!("{}", lexer.current_char);
//         lexer.next_char();
//     }
// }

fn main() -> () {
    // let source = "+- */ >>= = !=";
    // let source = "+- \"This is a string\" -- This is a comment!\n */";
    // let source = "+-123 9.8654*/";
    let source = "IF+-123 foo*THEN/";
    let mut lexer = Lexer::new(source.chars().collect());

    let mut token = lexer.get_token();
    while token.ttype != TokenType::EOF {
        println!("{}", token.ttype);
        token = lexer.get_token();
    }
}
