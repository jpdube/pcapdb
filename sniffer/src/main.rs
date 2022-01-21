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
    println!("Packet capture");
    pql::run();
    let config: Config = config::read(&"./config/config.yaml");
    println!("Db path: {}", config.db_path);
    // let database: MDatabase = MDatabase::new();
    // database.init();


    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("Target device is: {}", args[1]);
        sniffer::capture(&args[1]).unwrap();
    }
    println!("{:?}", args);
    // match database() {
    // Err(e) => println!("Error, you fucked up JP!!! {:?}", e),
    // Ok(()) => {}
    // }
}
