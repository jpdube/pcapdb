use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct DBConfig {
    filename_seq: u32,
}

impl DBConfig {
    pub fn new () -> Self {
        Self {
            filename_seq: 0
        }
    }

    pub fn read(&mut self) -> u32 {
        let f = std::fs::File::open("./db/db_config.yaml").expect("Could not open file.");
        let config: DBConfig = serde_yaml::from_reader(f).expect("Could not read values.");
        
        // self.pcap_file_size = config.pcap_file_size;
        println!("DBConfig.: {:?}", config);

        self.filename_seq = config.filename_seq + 1;
        self.write();

        config.filename_seq

    }

    fn write(&self) {
        let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("./db/db_config.yaml")
        .expect("Couldn't open file");
        
        serde_yaml::to_writer(f, &self).unwrap();
    }
}
