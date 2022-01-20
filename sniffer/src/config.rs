use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub db_path: String,
    pub pcap_file_size: u32,
    pub retention_time: String
}


pub fn read(config_file: &str) -> Config {
    let f = std::fs::File::open(config_file).expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    println!("{:?}", config);

    config
}                         

