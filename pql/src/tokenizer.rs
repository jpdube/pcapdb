use std::collections::HashMap;
use std::io::Read;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Select,
    From,
    Where,
    Commit,
    Interface,
    Between,
    OrderAsc,
    OrderDesc,
    Limit,
    Offset,
    Now,
    Land,
    Lor,
    Band,
    Bor,
    Equal,
    Mask,
    Minus,
    Plus,
    Star,
    Lt,
    Gt,
    Lparen,
    Rparen,
    Comma,
    Le,
    Ge,
    IpV4,
    Float,
    Integer,
    Date,
    Time,
    Identifier,
    Eol,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: Keyword,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

// Load the file to parse
pub fn load(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut lines = String::new();
    file.read_to_string(&mut lines).unwrap();

    return lines;
}

fn init_token_one(keywords: &mut HashMap<&str, Keyword>) {
    keywords.insert("=", Keyword::Equal);
    keywords.insert("/", Keyword::Mask);
    keywords.insert(";", Keyword::Eol);
    keywords.insert("-", Keyword::Minus);
    keywords.insert("+", Keyword::Plus);
    keywords.insert("*", Keyword::Star);
    keywords.insert("<", Keyword::Lt);
    keywords.insert(">", Keyword::Gt);
    keywords.insert("(", Keyword::Lparen);
    keywords.insert(")", Keyword::Rparen);
    keywords.insert(",", Keyword::Comma);
}

fn init_token_two(keywords: &mut HashMap<&str, Keyword>) {
    keywords.insert("<=", Keyword::Le);
    keywords.insert(">=", Keyword::Ge);
}

fn init_keywords(keywords: &mut HashMap<&str, Keyword>) {
    keywords.insert("select", Keyword::Select);
    keywords.insert("from", Keyword::From);
    keywords.insert("where", Keyword::Where);
    keywords.insert("commit", Keyword::Commit);
    keywords.insert("interface", Keyword::Interface);
    keywords.insert("and", Keyword::Land);
    keywords.insert("or", Keyword::Lor);
    keywords.insert("between", Keyword::Between);
    keywords.insert("order_asc", Keyword::OrderAsc);
    keywords.insert("order_desc", Keyword::OrderDesc);
    keywords.insert("limit", Keyword::Limit);
    keywords.insert("offset", Keyword::Offset);
    keywords.insert("now", Keyword::Now);
}

pub fn tokenize(lines: &str) -> Vec<Token> {
    let mut index = 0;
    let s: Vec<_> = lines.chars().collect();
    let mut token_list: Vec<Token> = Vec::new();
    let mut keywords = HashMap::new();
    let mut token_one = HashMap::new();
    let mut token_two = HashMap::new();

    let mut line: usize = 1;
    let mut line_offset: usize = 0;

    init_keywords(&mut keywords);
    init_token_one(&mut token_one);
    init_token_two(&mut token_two);

    while index < s.len() {
        if s[index].is_whitespace() {
            if s[index] == '\n' {
                let token = Token {
                    token: Keyword::Eol,
                    value: String::from("eol"),
                    column: (index + 1) - line_offset,
                    line: line,
                };
                token_list.push(token);
                line += 1;
                line_offset = index + 1;
            }
            index += 1;
        } else if (index + 2) <= s.len()
            && token_two.contains_key(&String::from_iter(s[index..index + 2].to_vec()) as &str)
        {
            let stoken: &str = &String::from_iter(s[index..index + 2].to_vec());
            let tok = token_two.get(stoken).unwrap();
            let token = Token {
                token: tok.to_owned(),
                value: String::from(stoken),
                column: (index + 1) - line_offset,
                line,
            };
            token_list.push(token);

            index += 2;
        } else if token_one.contains_key(&String::from(s[index]) as &str) {
            let tok = token_one.get(&String::from(s[index]) as &str).unwrap();
            let token = Token {
                token: tok.to_owned(),
                value: String::from(s[index]),
                column: (index + 1) - line_offset,
                line,
            };
            token_list.push(token);

            if s[index] == ';' {
                line += 1;
            }

            index += 1;
        } else if s[index].is_numeric() || s[index] == '.' || s[index] == '-' || s[index] == ':' {
            let start = index;

            while index < s.len()
                && (s[index].is_numeric() || s[index] == '.' || s[index] == '-' || s[index] == ':')
            {
                index += 1;
            }

            let number = String::from_iter(s[start..index].to_vec());
            if number.matches(".").count() == 3 {
                let token = Token {
                    token: Keyword::IpV4,
                    value: number,
                    column: (start + 1) - line_offset,
                    line,
                };
                token_list.push(token);
            } else if number.contains(".") {
                let token = Token {
                    token: Keyword::Float,
                    value: number,
                    column: (start + 1) - line_offset,
                    line,
                };
                token_list.push(token);
            } else if number.matches("-").count() == 2 {
                let token = Token {
                    token: Keyword::Date,
                    value: number,
                    column: (start + 1) - line_offset,
                    line,
                };
                token_list.push(token);
            } else if number.matches(":").count() == 2 {
                let token = Token {
                    token: Keyword::Time,
                    value: number,
                    column: (start + 1) - line_offset,
                    line,
                };
                token_list.push(token);
            } else {
                let token = Token {
                    token: Keyword::Integer,
                    value: number,
                    column: (start + 1) - line_offset,
                    line,
                };
                token_list.push(token);
            }
        } else if s[index].is_alphabetic() || s[index] == '_' {
            let start = index;
            while index < s.len()
                && (s[index].is_alphabetic() || s[index].is_numeric() || s[index] == '_')
            {
                index += 1;
            }
            let keyword: &str = &String::from_iter(s[start..index].to_vec());
            if keywords.contains_key(keyword) {
                let tok = keywords.get(&keyword).unwrap();
                let token = Token {
                    token: tok.to_owned(),
                    value: String::from(keyword),
                    column: (start + 1) - line_offset,
                    line,
                };
                token_list.push(token);
            } else {
                let token = Token {
                    token: Keyword::Identifier,
                    value: String::from(keyword),
                    column: (start + 1) - line_offset,
                    line,
                };
                token_list.push(token);
            }
        } else {
            index += 1;
        }
    }

    let token = Token {
        token: Keyword::Eof,
        value: String::from("eof"),
        column: (index + 1) - line_offset,
        // column: s.len() + 1,
        line: line,
    };
    token_list.push(token);

    return token_list;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_token() {
        let line = "select ip_src, ip_dst from sniffer_01";
        let token_list: Vec<Token> = tokenize(line);
        assert!(token_list.len() == 7);
    }

    #[test]
    fn date_token() {
        let line = "12-01-2022";
        let token_list: Vec<Token> = tokenize(line);

        assert!(token_list[0].token == Keyword::Date);
    }

    #[test]
    fn time_token() {
        let line = "14:33:56";
        let token_list: Vec<Token> = tokenize(line);

        assert!(token_list[0].token == Keyword::Time);
    }

    #[test]
    fn ipv4_cidr_mask() {
        let line = "192.168.0.0/24";
        let token_list: Vec<Token> = tokenize(line);
        assert!(token_list.len() == 4);

        assert!(token_list[0].token == Keyword::IpV4);
        assert!(token_list[1].token == Keyword::Mask);
        assert!(token_list[2].token == Keyword::Integer);
    }

    #[test]
    fn ipv4_byte_mask() {
        let line = "192.168.0.0 / 255.255.255.0";
        let token_list: Vec<Token> = tokenize(line);
        assert!(token_list.len() == 4);

        assert!(token_list[0].token == Keyword::IpV4);
        assert!(token_list[1].token == Keyword::Mask);
        assert!(token_list[2].token == Keyword::IpV4);
    }

    #[test]
    fn column_no() {
        let line = "select ip_dst, ip_src from sniffer_01 where dport = 443";
        let tl: Vec<Token> = tokenize(line);
        assert!(tl.len() == 11);

        assert!(tl[0].token == Keyword::Select && tl[0].column == 1 && tl[0].line == 1);
        assert!(tl[1].token == Keyword::Identifier && tl[1].column == 8 && tl[1].line == 1);
        assert!(tl[4].token == Keyword::From && tl[4].column == 23 && tl[4].line == 1);
        assert!(tl[6].token == Keyword::Where && tl[6].column == 39 && tl[6].line == 1);
    }

    #[test]
    fn multiline() {
        let line = "select ip_dst, ip_src\nfrom sniffer_01\nwhere dport = 443";
        let tl: Vec<Token> = tokenize(line);
        assert!(tl.len() == 13);

        assert!(tl[0].token == Keyword::Select && tl[0].column == 1 && tl[0].line == 1);
        assert!(tl[1].token == Keyword::Identifier && tl[1].column == 8 && tl[1].line == 1);
        assert!(tl[5].token == Keyword::From && tl[5].column == 1 && tl[5].line == 2);
        assert!(tl[8].token == Keyword::Where && tl[8].column == 1 && tl[8].line == 3);
    }

    #[test]
    fn two_chars_tokens() {
        let line = ">= <=";
        let tl: Vec<Token> = tokenize(line);
        assert!(tl.len() == 3);

        assert!(tl[0].token == Keyword::Ge && tl[0].column == 1 && tl[0].line == 1);
        assert!(tl[1].token == Keyword::Le && tl[1].column == 4 && tl[1].line == 1);
    }

    #[test]
    fn one_chars_tokens() {
        let line = "< > = - + * / , ;";
        let tl: Vec<Token> = tokenize(line);
        assert!(tl.len() == 10);

        assert!(tl[0].token == Keyword::Lt && tl[0].column == 1 && tl[0].line == 1);
        assert!(tl[1].token == Keyword::Gt && tl[1].column == 3 && tl[1].line == 1);
        assert!(tl[2].token == Keyword::Equal && tl[2].column == 5 && tl[2].line == 1);
        assert!(tl[3].token == Keyword::Minus && tl[3].column == 7 && tl[3].line == 1);
        assert!(tl[4].token == Keyword::Plus && tl[4].column == 9 && tl[4].line == 1);
        assert!(tl[5].token == Keyword::Star && tl[5].column == 11 && tl[5].line == 1);
        assert!(tl[6].token == Keyword::Mask && tl[6].column == 13 && tl[6].line == 1);
        assert!(tl[7].token == Keyword::Comma && tl[7].column == 15 && tl[7].line == 1);
        assert!(tl[8].token == Keyword::Eol && tl[8].column == 17 && tl[8].line == 1);
    }

    #[test]
    fn grouping() {
        let line = "()";
        let tl: Vec<Token> = tokenize(line);
        assert!(tl.len() == 3);

        assert!(tl[0].token == Keyword::Lparen && tl[0].column == 1 && tl[0].line == 1);
        assert!(tl[1].token == Keyword::Rparen && tl[1].column == 2 && tl[1].line == 1);
    }
}
