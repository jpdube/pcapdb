use std::env;

mod config;
mod db_config;
mod dbmongo;
mod packetref;
mod sniffer;
use config::Config;
use dbmongo::MDatabase;
use pql;

#[warn(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Packet capture");
    println!("{:?}", args);
    pql::run();
    let config: Config = config::read(&args[1]);
    println!("Db path: {}", config.db_path);
    let database: MDatabase = MDatabase::new();
    database.init();


    if args.len() == 3 {
        println!("Target device is: {}", args[2]);
        sniffer::capture(&args[2]).unwrap();
    }
}
