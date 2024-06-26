use std::collections::HashSet;

use crate::lexer;
use crate::emitter;

use lexer::{Lexer, Token, TokenType};
use emitter::Emitter;

/**
 * Parser object keeps track of current token and checks if the code matches the grammar.
 */
pub struct Parser<'a> {
    lexer: Lexer,
    emitter: &'a mut Emitter,
    current_token: Option::<Token>,
    peek_token: Option::<Token>,

    symbols: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_used: HashSet<String>,
}

impl<'a> Parser<'a> {
    pub fn new (lexer: Lexer, emitter: &'a mut Emitter) -> Self {
        let mut parser = Parser {
            lexer, emitter,
            current_token: None,
            peek_token: None,
            symbols: HashSet::new(),
            labels_declared: HashSet::new(),
            labels_used: HashSet::new(),
        };

        // Calling twice to initialize current and peek tokens.
        parser.next_token();
        parser.next_token();
        
        return parser;
    }

    /**
     * Return true if the current token matches.
     */
    fn check_token(&self, token_type: TokenType) -> bool {
        return self.current_token.as_ref().unwrap().ttype == token_type;
    }

    /**
     * Try to match current token. If not, error. Advances the current token.
     */
    fn try_match(&mut self, token_type: TokenType) -> () {
        if ! (self.check_token(token_type)) {
            let cur_token = self.current_token.clone().unwrap().ttype.to_string();
            let expected_token = token_type.to_string();
            self.abort(format!("Expected {cur_token}, got {expected_token}"));
        }

        self.next_token();
    }

    /**
     * Advances the current token.
     */
    fn next_token(&mut self) -> () {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.get_token());
    }

    /**
     * Get the text of the current token
     */
    fn get_current_token_text(&self) -> String {
        return self.current_token.as_ref().unwrap().text.clone();
    }

    /**
     * Get the type of the current token
     */
    fn get_current_token_type(&self) -> TokenType {
        return self.current_token.as_ref().unwrap().ttype;
    }

    fn abort(&self, message: String) -> () {
        println!("Aborted! {}", message);
        std::process::exit(1)
    }
    
    pub fn program(&mut self) {
        println!("PROGRAM");
        self.emitter.header_line("#include <stdio.h>");
        self.emitter.header_line("int main (void) {");

        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }

        // Parse all the statements in the program
        while ! self.check_token(TokenType::EOF) {
            self.statement();
        };

        self.emitter.emit_line("return 0;");
        self.emitter.emit_line("}");

        for label in &self.labels_used {
            if ! self.labels_declared.contains(label) {
                self.abort(format!("Attempting to GOTO to undeclared label: {label}"));
            }
        }
    }

    /**
     * Onew of the following statements
     */
    fn statement (&mut self) {
        match self.get_current_token_type() {
            // "PRINT" (expression | string)
            TokenType::PRINT => {
                println!("STATEMENT-PRINT");
                self.next_token();

                if self.check_token(TokenType::STRING) {
                    let token_text = self.get_current_token_text();
                    self.emitter.emit_line(format!("printf(\"{token_text}\\n\");").as_str());
                    self.next_token();
                } else {
                    self.emitter.emit("printf(\"%.2f\\n\", (float)(");
                    self.expression();
                    self.emitter.emit_line("));");
                }
            },

            // "IF" comparison "THEN" {statement} "ENDIF"
            TokenType::IF => {
                println!("STATEMENT-IF");
                self.next_token();

                self.emitter.emit("if(");

                self.comparison();
                self.try_match(TokenType::THEN);
                self.new_line();

                self.emitter.emit_line(") {");

                while ! self.check_token(TokenType::ENDIF) {
                    self.statement();
                }

                self.try_match(TokenType::ENDIF);
                self.emitter.emit_line("}");
            },

            // "WHILE" comparison "REPEAT" {statement} "ENDWHILE"
            TokenType::WHILE => {
                println!("STATEMENT-WHILE");

                self.next_token();

                self.emitter.emit("while(");

                self.comparison();

                self.try_match(TokenType::REPEAT);
                self.new_line();

                self.emitter.emit_line(") {");

                // Zero or more statements in the loop body.
                while ! self.check_token(TokenType::ENDWHILE) {
                    self.statement();
                }

                self.try_match(TokenType::ENDWHILE);
                self.emitter.emit_line("}");
            },

            // "LABEL" ident
            TokenType::LABEL => {
                println!("STATEMENT-LABEL");

                let token_text = self.get_current_token_text();
                
                // Make sure this label already doesn't exist already.
                if self.labels_declared.contains(&token_text) {
                    self.abort(format!("Label already exists: {token_text}"));
                }

                self.labels_declared.insert(token_text.clone());

                // self.next_token();
                self.try_match(TokenType::IDENT);
                self.emitter.emit_line(format!("{token_text}:").as_str())
            },

            // "GOTO" ident
            TokenType::GOTO => {
                println!("STATEMENT-GOTO");
                self.next_token();
                
                let token_text = self.current_token.as_ref().unwrap().text.clone();
                self.labels_used.insert(token_text.clone());

                self.try_match(TokenType::IDENT);
                self.emitter.emit_line(format!("goto {token_text};").as_str());
            },

            // "LET" ident
            TokenType::LET => {
                println!("STATEMENT-LET");
                self.next_token();

                self.declare_current_symbol();

                let token_text = self.get_current_token_text();
                self.emitter.emit(format!("{token_text} = ").as_str());

                self.try_match(TokenType::IDENT);
                self.try_match(TokenType::EQ);
                self.expression();
                self.emitter.emit_line(";");
            },

            // "INPUT" ident
            TokenType::INPUT => {
                println!("STATEMENT-INPUT");
                self.next_token();

                self.declare_current_symbol();

                let token_text = self.get_current_token_text();
                self.try_match(TokenType::IDENT);

                self.emitter.emit_line(format!("if (0 == scanf(\"%f\", &{token_text})) {{").as_str());
                self.emitter.emit_line(format!("{token_text} = 0;").as_str());
                self.emitter.emit("scanf(\"%");
                self.emitter.emit_line("*s\");");
                self.emitter.emit_line("}");
            },

            _ => {
                let cur_token_text = self.get_current_token_text();
                let cur_token_type_text = self.get_current_token_type().to_string();

                self.abort(format!("Invalid statement at {cur_token_text} ({cur_token_type_text})"));
            },
        };

        // New Line
        self.new_line();
    }

    fn new_line(&mut self) {
        println!("NEW LINE");

        // Require at least one new line.
        self.try_match(TokenType::NEWLINE);

        // But we will allow extra new lines too, of course.
        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }
    }

    // comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)
    fn comparison(&mut self) {
        println!("COMPARISON");
        self.expression();

        // Must be at least one comparison operator and another expression.
        if self.is_comparison_operator() {
            self.handle_comparision();
        } else {
            let token_text = self.get_current_token_text();
            self.abort(format!("Expected comparison operator at: {token_text}"));
        }

        // Can have 0 or more comparison operator and expressions.
        while self.is_comparison_operator() {
            self.handle_comparision();
        }
    }

    fn is_comparison_operator(&self) -> bool {
        return [
            TokenType::GT, TokenType::GTEQ,
            TokenType::LT, TokenType::LTEQ,
            TokenType::EQEQ, TokenType::NOTEQ
        ].contains(&self.get_current_token_type());
    }

    fn handle_comparision(&mut self) {
        self.emitter.emit(self.get_current_token_text().as_str());
        self.next_token();
        self.expression();
    }

    // expression ::= term {( "-" | "+" ) term}
    fn expression(&mut self) {
        println!("EXPRESSION");

        self.term();

        // can have 0 or more +/- expressions.
        while self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.emitter.emit(self.get_current_token_text().as_str());
            self.next_token();
            self.term();
        }
    }

    // term ::= unary {( "/" | "*" ) unary}
    fn term(&mut self) {
        println!("TERM");

        self.unary();

        // can have 0 or more * or / expressions.
        while self.check_token(TokenType::ASTERISK) || self.check_token(TokenType::SLASH) {
            self.emitter.emit(self.get_current_token_text().as_str());
            self.next_token();
            self.unary();
        }
    }

    // unary ::= ["+" | "-"] primary
    fn unary(&mut self) {
        println!("UNARY");

        // Optional unary +/-
        if self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.emitter.emit(self.get_current_token_text().as_str());
            self.next_token();
        }

        self.primary();
    }

    // primary ::= number | ident
    fn primary(&mut self) {
        let cur_token = self.current_token.as_ref().unwrap();
        let cur_token_text = cur_token.text.clone();
        println!("PRIMARY ({cur_token_text})");

        match cur_token.ttype {
            TokenType::NUMBER => {
                self.emitter.emit(self.get_current_token_text().as_str());
                self.next_token();
            },

            TokenType::IDENT => {
                let token_text = self.get_current_token_text();
                if ! self.symbols.contains(&token_text) {
                    self.abort(format!("Referencing variable berfore assignment: {token_text}"));
                }

                self.emitter.emit(self.get_current_token_text().as_str());
                self.next_token();
            },

            _ => {
                self.abort(format!("Unexpected token at {cur_token_text}"));
            }
        }
    }

    /**
     * Check if ident exists in symbol table. If not, declare it.
     */         
    fn declare_current_symbol(&mut self) {
        let token_text = self.get_current_token_text();

        if ! self.symbols.contains(&token_text) {
            self.symbols.insert(token_text.clone());
            self.emitter.header_line(format!("float {token_text};").as_str());
        }
    }
}