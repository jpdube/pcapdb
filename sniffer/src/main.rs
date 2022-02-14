use std::env;

mod config;
mod db_config;
mod packetref;
mod sniffer;
mod db;
use config::Config;
use db::Database;
// use pql;

#[warn(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();
    about();
    println!("{:?}", args);
    // pql::run();

    let config: Config = config::read(&args[1]);
    println!("Db path: {}", config.db_path);

    if args.len() == 3 {
        println!("Target device is: {}", args[2]);
        sniffer::capture(&args[2]).unwrap();
    }
}

fn about() {
    println!("");
    println!("PCAPDB Packet database and collector");
    println!("Numa Informatique Inc");
    println!("Version 0.1");
    println!("------------------------------------");
}
