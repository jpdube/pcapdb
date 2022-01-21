use std::collections::HashMap;
use std::io::Read;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Token {
    pub token: String,
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

fn init_token_one(keywords: &mut HashMap<&str, &str>) {
    keywords.insert("=", "EQUAL");
    keywords.insert("/", "MASK");
    keywords.insert(";", "EOL");
}

fn init_keywords(keywords: &mut HashMap<&str, &str>) {
    keywords.insert("select", "SELECT");
    keywords.insert("from", "FROM");
    keywords.insert("where", "WHERE");
    keywords.insert("commit", "COMMIT");
    keywords.insert("interface", "INTERFACE");
    keywords.insert("and", "AND");
    keywords.insert("or", "OR");
    keywords.insert("between", "BETWEEN");
    keywords.insert("order_asc", "ORDER ASC");
    keywords.insert("order_desc", "ORDER DESC");
    keywords.insert("limit", "LIMIT");
    keywords.insert("offset", "OFFSET");
}

pub fn tokenize(lines: &str) -> Vec<Token> {
    let mut index = 0;
    let s: Vec<_> = lines.chars().collect();
    let mut token_list: Vec<Token> = Vec::new();
    let mut keywords = HashMap::new();
    let mut token_one = HashMap::new();

    let mut line: usize = 1;

    init_keywords(&mut keywords);
    init_token_one(&mut token_one);

    while index < s.len() {
        if s[index].is_whitespace() {
            if s[index] == '\n' {
                let token = Token {
                    token: String::from("EOL"),
                    value: String::from("eol"),
                    column: index,
                    line: line,
                };
                token_list.push(token);
                line += 1;
            }
            index += 1;
        } else if token_one.contains_key(&String::from(s[index]) as &str) {
            let tok = token_one.get(&String::from(s[index]) as &str).unwrap();
            let token = Token {
                token: String::from(tok.to_string()),
                value: String::from(s[index]),
                column: index,
                line,
            };
            token_list.push(token);
            
            if s[index] == ';' {
                line += 1;
            }

            index += 1;
        } else if s[index].is_numeric() || s[index] == '.' {
            let start = index;

            while index < s.len() && (s[index].is_numeric() || s[index] == '.') {
                index += 1;
            }

            let number = String::from_iter(s[start..index].to_vec());
            if number.matches(".").count() == 3 {
                let token = Token {
                    token: String::from("IPV4"),
                    value: number,
                    column: start,
                    line
                };
                token_list.push(token);
            } else if number.contains(".") {
                let token = Token {
                    token: String::from("FLOAT"),
                    value: number,
                    column: start,
                    line
                };
                token_list.push(token);
            } else {
                let token = Token {
                    token: String::from("INTEGER"),
                    value: number,
                    column: start,
                    line
                };
                token_list.push(token);
            }
        } else if s[index].is_alphabetic()  || s[index] == '_' {
            let start = index;
            while index < s.len() && (s[index].is_alphabetic() || s[index].is_numeric() || s[index] == '_') {
                index += 1;
            }
            let keyword: &str = &String::from_iter(s[start..index].to_vec());
            if keywords.contains_key(keyword) {
                let tok = keywords.get(&keyword).unwrap();
                let token = Token {
                    token: String::from(tok.to_string()),
                    value: String::from(keyword),
                    column: start,
                    line
                };
                token_list.push(token);
            } else {
                let token = Token {
                    token: String::from("NAME"),
                    value: String::from(keyword),
                    column: start,
                    line
                };
                token_list.push(token);
            }
        } else {
            index += 1;
        }
    }

    let token = Token {
        token: String::from("EOF"),
        value: String::from("eof"),
        column: s.len(),
        line: line - 1
    };
    token_list.push(token);

    return token_list;
}



