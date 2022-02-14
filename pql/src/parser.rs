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

    pub fn run(&mut self) {
        while self.peek(Keyword::Semi).is_none() {
            if self.accept(Keyword::Select).is_some() {
                println!("***FOUND SELECT");
                //--- Get fields
                while self.peek(Keyword::Identifier).is_some() {
                    let name = self.expect(Keyword::Identifier);
                    if name.is_some() {
                        println!("Field {:#?}", name.unwrap())
                    }
                    if self.accept(Keyword::Comma).is_none() {
                        break;
                    }
                }
                //--- Get from
                if self.accept(Keyword::From).is_some() {
                    println!("***FOUND FROM");
                    //--- Get fields
                    while self.peek(Keyword::Identifier).is_some() {
                        let name = self.expect(Keyword::Identifier);
                        if name.is_some() {
                            println!("Field {:#?}", name.unwrap())
                        }
                        if self.accept(Keyword::Comma).is_none() {
                            break;
                        }
                    }

                }
            }
        }
    }

    pub fn execute(&mut self) {
        let mut columns: Vec<Token> = vec![];

        let s = self.expect(Keyword::Select);
        if s.is_some() {
            columns.push(s.unwrap());
            while self.peek(Keyword::Identifier).is_some() {
                let t = self.accept(Keyword::Identifier).unwrap();
                columns.push(t);
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

    pub fn parse_str(&mut self, source: &str) {
        self.source = source.to_string();

        self.tokens = tokenize(&self.source);
    }

    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn peek(&mut self, tlookup: Keyword) -> Option<Token> {
        if self.lookahead.is_none() {
            self.lookahead = self.next();
        }

        if self.lookahead.is_none() {
            None
        } else {
            if self.lookahead.as_ref().unwrap().token == tlookup {
                self.lookahead.clone()
            } else {
                None
            }
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

        // println!("Next index: {},{}", self.ptr, self.token_count());
        if self.ptr + 1 <= self.token_count() {
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
    fn test_run() {
        let mut parser = Parser::new();
        parser.parse_str("select ip_src, ip_dst from snif_01;");
        assert!(parser.token_count() == 8);
        parser.run();
    }

    #[test]
    fn load_file() {
        let mut parser = Parser::new();
        parser.parse_file("../examples/basic.pql");
        assert!(parser.token_count() == 10);
    }

    #[test]
    fn peek_one() {
        let mut parser = Parser::new();
        parser.parse_file("../examples/basic.pql");
        assert!(parser.token_count() == 10);
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
        assert!(parser.token_count() == 10);
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
        assert!(parser.token_count() == 10);
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
        assert!(parser.token_count() == 10);
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
        assert!(parser.token_count() == 12);
        parser.execute();
        assert!(true)
    }
}
