mod tokenizer;
use tokenizer::{tokenize, Token};

pub fn run() {

    let line: &str = "select ip_src, ip_dst from sniffer_01 where ip_src = 192.168.242.0/24 and ip_dst = 192.168.1.22 order_asc ip_src;select a from b;";
    let token_list: Vec<Token> = tokenize(line);

    println!("{}", line);
    for t in token_list {
        println!("{:?}", t);

    }
    // println!("{:?}", token_list);
}
