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
    keywords.insert("-", "MINUS");
    keywords.insert("+", "PLUS");
    keywords.insert("*", "MULTIPLY");
    keywords.insert("<", "LT");
    keywords.insert(">", "GT");
    keywords.insert("(", "LPAREN");
    keywords.insert(")", "RPAREN");
}

fn init_token_two(keywords: &mut HashMap<&str, &str>) {
    keywords.insert("<=", "LE");
    keywords.insert(">=", "GE");
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
    keywords.insert("now", "NOW");
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
                    token: String::from("EOL"),
                    value: String::from("eol"),
                    column: (index + 1) - line_offset,
                    line: line,
                };
                token_list.push(token);
                line += 1;
                line_offset = index + 1;
            }
            index += 1;
        } else if (index + 2) <= s.len() && token_two.contains_key(&String::from_iter(s[index..index + 2].to_vec()) as &str) {
            let stoken: &str = &String::from_iter(s[index..index + 2].to_vec());
            let tok = token_two.get(stoken).unwrap();
            let token = Token {
                token: String::from(tok.to_string()),
                value: String::from(stoken),
                column: (index + 1) - line_offset,
                line,
            };
            token_list.push(token);
            
            index += 2;
        }
        else if token_one.contains_key(&String::from(s[index]) as &str) {
            let tok = token_one.get(&String::from(s[index]) as &str).unwrap();
            let token = Token {
                token: String::from(tok.to_string()),
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

            while index < s.len() && (s[index].is_numeric() || s[index] == '.' || s[index] == '-' || s[index] == ':') {
                index += 1;
            }

            let number = String::from_iter(s[start..index].to_vec());
            if number.matches(".").count() == 3 {
                let token = Token {
                    token: String::from("IPV4"),
                    value: number,
                    column: (start + 1) - line_offset,
                    line
                };
                token_list.push(token);
            } else if number.contains(".") {
                let token = Token {
                    token: String::from("FLOAT"),
                    value: number,
                    column: (start + 1) - line_offset,
                    line
                };
                token_list.push(token);
            } else if number.matches("-").count() == 2 {
                let token = Token {
                    token: String::from("DATE"),
                    value: number,
                    column: (start + 1) - line_offset,
                    line
                };
                token_list.push(token);
            } else if number.matches(":").count() == 2 {
                let token = Token {
                    token: String::from("TIME"),
                    value: number,
                    column: (start + 1) - line_offset,
                    line
                };
                token_list.push(token);
            } else {
                let token = Token {
                    token: String::from("INTEGER"),
                    value: number,
                    column: (start + 1) - line_offset,
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
                    column: (start + 1) - line_offset,
                    line
                };
                token_list.push(token);
            } else {
                let token = Token {
                    token: String::from("NAME"),
                    value: String::from(keyword),
                    column: (start + 1) - line_offset,
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
        column: (index + 1) - line_offset,
        // column: s.len() + 1,
        line: line
    };
    token_list.push(token);

    return token_list;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_token() {
        let line: &str = "select ip_src, ip_dst from sniffer_01";
        let token_list: Vec<Token> = tokenize(line);
        assert!(token_list.len() == 6);
    }

    #[test]
    fn date_token() {
        let line: &str = "12-01-2022";
        let token_list: Vec<Token> = tokenize(line);

        assert!(token_list[0].token == "DATE");
    }

    #[test]
    fn time_token() {
        let line: &str = "14:33:56";
        let token_list: Vec<Token> = tokenize(line);

        assert!(token_list[0].token == "TIME");
    }


    #[test]
    fn ipv4_cidr_mask() {
        let line: &str = "192.168.0.0/24";
        let token_list: Vec<Token> = tokenize(line);
        assert!(token_list.len() == 4);

        assert!(token_list[0].token == "IPV4");
        assert!(token_list[1].token == "MASK");
        assert!(token_list[2].token == "INTEGER");
    }

    #[test]
    fn ipv4_byte_mask() {
        let line: &str = "192.168.0.0 / 255.255.255.0";
        let token_list: Vec<Token> = tokenize(line);
        assert!(token_list.len() == 4);

        assert!(token_list[0].token == "IPV4");
        assert!(token_list[1].token == "MASK");
        assert!(token_list[2].token == "IPV4");
    }

    #[test]
    fn column_no() {
        let line: &str = "select ip_dst, ip_src from sniffer_01 where dport = 443";
        let tl: Vec<Token> = tokenize(line);
        assert!(tl.len() == 10);

        assert!(tl[0].token == "SELECT" && tl[0].column == 1 && tl[0].line == 1);
        assert!(tl[1].token == "NAME" && tl[1].column == 8 && tl[1].line == 1);
        assert!(tl[3].token == "FROM" && tl[3].column == 23 && tl[3].line == 1);
        assert!(tl[5].token == "WHERE" && tl[5].column == 39 && tl[5].line == 1);
    }

    #[test]
    fn multiline() {
        let line: &str = "select ip_dst, ip_src\nfrom sniffer_01\nwhere dport = 443";
        let tl: Vec<Token> = tokenize(line);
        assert!(tl.len() == 12);

        assert!(tl[0].token == "SELECT" && tl[0].column == 1 && tl[0].line == 1);
        assert!(tl[1].token == "NAME" && tl[1].column == 8 && tl[1].line == 1);
        assert!(tl[4].token == "FROM" && tl[4].column == 1 && tl[4].line == 2);
        assert!(tl[7].token == "WHERE" && tl[7].column == 1 && tl[7].line == 3);
    }

    #[test]
    fn two_chars_tokens() {
        let line: &str = ">= <=";
        let tl: Vec<Token> = tokenize(line);
        assert!(tl.len() == 3);

        assert!(tl[0].token == "GE" && tl[0].column == 1 && tl[0].line == 1);
        assert!(tl[1].token == "LE" && tl[1].column == 4 && tl[1].line == 1);
    }

    #[test]
    fn one_chars_tokens() {
        let line: &str = "< > = - + * / ;";
        let tl: Vec<Token> = tokenize(line);
        assert!(tl.len() == 9);

        assert!(tl[0].token == "LT" && tl[0].column == 1 && tl[0].line == 1);
        assert!(tl[1].token == "GT" && tl[1].column == 3 && tl[1].line == 1);
        assert!(tl[2].token == "EQUAL" && tl[2].column == 5 && tl[2].line == 1);
        assert!(tl[3].token == "MINUS" && tl[3].column == 7 && tl[3].line == 1);
        assert!(tl[4].token == "PLUS" && tl[4].column == 9 && tl[4].line == 1);
        assert!(tl[5].token == "MULTIPLY" && tl[5].column == 11 && tl[5].line == 1);
        assert!(tl[6].token == "MASK" && tl[6].column == 13 && tl[6].line == 1);
        assert!(tl[7].token == "EOL" && tl[7].column == 15 && tl[7].line == 1);
    }

    #[test]
    fn grouping() {
        let line: &str = "()";
        let tl: Vec<Token> = tokenize(line);
        assert!(tl.len() == 3);

        assert!(tl[0].token == "LPAREN" && tl[0].column == 1 && tl[0].line == 1);
        assert!(tl[1].token == "RPAREN" && tl[1].column == 2 && tl[1].line == 1);
    }

}

