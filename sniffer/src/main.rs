use std::env;

mod config;
mod db;
mod db_config;
mod packetref;
mod pql_api;
mod sniffer;
mod print_hex;

use config::Config;
use std::thread;
use ctrlc;
use std::process::exit;
// use pql;

#[warn(dead_code)]
fn main() {
    ctrlc::set_handler(move || {
        println!("Received Ctrl+C, pcapdb is terminating");
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    
    let args: Vec<String> = env::args().collect();
    about();

    pql_api::start();

    println!("{:?}", args);

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
