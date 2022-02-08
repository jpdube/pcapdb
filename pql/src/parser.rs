use std::error::Error;
use std::fs;
use std::io::Read;

use crate::tokenizer::{tokenize, Keyword, Token};

/*
    Goals of the parser and executer

    1- Index scan
        a- Process the pql and derive an sql statement to use
           for the index search in the database.

        b- Build a where clause for the index search

        c- Execute the query
        d- Send the result of packet positions to step 2

    2- Table scan
        a- From the result obtain from step 1
        b- Build and run the filter for the packet scan

*/

pub struct Parser {
    filename: String,
    source: String,
    tokens: Vec<Token>,
    ptr: usize,
    lookahead: Option<Token>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            filename: String::from(""),
            source: String::from(""),
            tokens: Vec::new(),
            ptr: 0,
            lookahead: None,
        }
    }

    pub fn execute(&mut self) {
        let mut columns: Vec<Token> = vec![];

        if self.expect(Keyword::Select).is_some() {
            while self.peek(Keyword::Identifier).is_some() {
                let t = self.accept(Keyword::Identifier).unwrap();
                columns.push(t.clone());
                if self.accept(Keyword::Comma).is_none() {
                    break;
                }
            }
        }
        println!("Select field: {:?}", columns);
    }

    pub fn parse_file(&mut self, filename: &str) {
        self.filename = filename.to_string();
        self.source = fs::read_to_string(filename).unwrap().parse().unwrap();

        self.tokens = tokenize(&self.source);
    }

    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn peek(&mut self, tlookup: Keyword) -> Option<Token> {
        if self.lookahead.is_none() {
            self.lookahead = Some(self.next().unwrap());
        }

        if self.lookahead.as_ref().unwrap().token == tlookup {
            self.lookahead.clone()
        } else {
            None
        }
    }

    pub fn accept(&mut self, tlookup: Keyword) -> Option<Token> {
        let token = self.peek(tlookup);

        if token.is_some() {
            self.lookahead = None
        }

        token
    }

    pub fn expect(&mut self, tlookup: Keyword) -> Option<Token> {
        let token = self.peek(tlookup.clone());

        if token.is_none() {
            let lookup = self.lookahead.as_ref().unwrap();
            println!(
                "Syntax error expecting {:?} found {} at {}:{}",
                tlookup, lookup.value, lookup.line, lookup.column
            );
            None
        } else {
            self.lookahead = None;
            token
        }
    }

    fn next(&mut self) -> Option<Token> {
        let token: Option<Token> = Some(self.tokens[self.ptr].clone());

        if self.ptr + 1 < self.token_count() {
            self.ptr += 1;
            token
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file() {
        let mut parser = Parser::new();
        parser.parse_file("../examples/basic.pql");
        assert!(parser.token_count() == 11);
    }

    #[test]
    fn peek_one() {
        let mut parser = Parser::new();
        parser.parse_file("../examples/basic.pql");
        assert!(parser.token_count() == 11);
        match parser.peek(Keyword::Select) {
            Some(token) => {
                // println!("Token peek: {}", token.value);
                assert!(true)
            }
            None => {
                // println!("Token peek not found");
                assert!(false)
            }
        }
    }
    #[test]
    fn peek_two() {
        let mut parser = Parser::new();
        parser.parse_file("../examples/basic.pql");
        assert!(parser.token_count() == 11);
        match parser.peek(Keyword::Select) {
            Some(token) => {
                // println!("Token peek: {}", token.value);
                parser.accept(Keyword::Select);
                assert!(true)
            }
            None => {
                // println!("Token peek not found");
                assert!(false)
            }
        }
        match parser.peek(Keyword::Identifier) {
            Some(token) => {
                // println!("Token peek: {}", token.value);
                assert!(true)
            }
            None => {
                // println!("Token peek not found");
                assert!(false)
            }
        }
    }

    #[test]
    fn expect_one_token() {
        let mut parser = Parser::new();
        parser.parse_file("../examples/basic.pql");
        assert!(parser.token_count() == 11);
        match parser.expect(Keyword::From) {
            Some(token) => {
                // println!("Token peek: {}", token.value);
                assert!(false)
            }
            None => {
                // println!("Expected token not found");
                assert!(true)
            }
        }
    }

    #[test]
    fn expect_two_token() {
        let mut parser = Parser::new();
        parser.parse_file("../examples/basic.pql");
        assert!(parser.token_count() == 11);
        match parser.accept(Keyword::Select) {
            Some(token) => {
                // println!("Token peek: {}", token.value);
                assert!(true)
            }
            None => {
                // println!("Expected token not found");
                assert!(false)
            }
        }

        match parser.expect(Keyword::From) {
            Some(token) => {
                // println!("Token peek: {}", token.value);
                assert!(false)
            }
            None => {
                // println!("Expected token not found");
                assert!(true)
            }
        }
    }

    #[test]
    fn execute() {
        let mut parser = Parser::new();
        parser.parse_file("../examples/execute_two_fields.pql");
        assert!(parser.token_count() == 13);
        parser.execute();
        assert!(true)
    }

    #[test]
    fn test_str_cmp() {
        let str_a: &str = "This is string a";
        let str_b: &str = "This is string a";

        assert!(str_a == str_b)
    }
}
