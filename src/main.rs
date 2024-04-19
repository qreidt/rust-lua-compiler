mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use std::fs;

fn main() -> () {
    // let source = "+- */ >>= = !=";
    // let source = "+- \"This is a string\" -- This is a comment!\n */";
    // let source = "+-123 9.8654*/";
    // let source = "IF+-123 foo*THEN/";
    
    let contents = fs::read_to_string("./test.program")
        .expect("Should have been able to read the file");

    let lexer = Lexer::new(contents.chars().collect());
    let mut parser = Parser::new(lexer);

    parser.program();
    println!("Parsing Complete.")
}
