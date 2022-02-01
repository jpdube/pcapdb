mod tokenizer;
mod parser;
use tokenizer::{tokenize, Token};

pub fn run() {
    let line: &str = "select ip_dst, ip_src\nfrom sniffer_01\nwhere dport = 443 and (ip_src = 192.168.1.22 or ip_src = 192.168.1.23)\n";

    // let line: &str = "select ip_src, ip_dst\nfrom sniffer_01\nwhere ts = 12-02-2222 14:20:05 ip_src = 192.168.242.0/24 and ip_dst = 192.168.1.22\norder_asc ip_src and timestamp >= now - 1d;";
    let token_list: Vec<Token> = tokenize(line);

    println!("{}", line);
    for t in &token_list {
        println!("{:?}", t);

    }
}
