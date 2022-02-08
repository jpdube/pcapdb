use rusqlite::{named_params, Connection, Result, Transaction};

pub struct DbInfo {
    pub timestamp: u32,
    pub src_mac: u64,
    pub dst_mac: u64,
    pub ether_type: u16,
    pub ip_proto: u8,
    pub src_ip: u32,
    pub dst_ip: u32,
    pub sport: u16,
    pub dport: u16,
    pub pkt_ptr: u64,
    pub file_no: u32,
}

pub struct Database {
    conn: Connection,
    filename: String,
}

impl Database {
    pub fn save_many(&mut self, pkt_list: &Vec<DbInfo>) {
        let sql = "INSERT INTO packet (mac_src, mac_dst, ip_src, ip_dst, sport, dport, file_ptr, file_id, timestamp) values (:mac_src, :mac_dst, :ip_src, :ip_dst, :sport, :dport, :file_ptr, :file_id, :timestamp)";
        let mut stmt = self.conn.prepare(sql).unwrap();
        self.conn.execute("BEGIN TRANSACTION", []).unwrap();
        for pkt in pkt_list.iter() {
            stmt.execute(named_params! {
                ":mac_src": pkt.src_mac,
                ":mac_dst": pkt.dst_mac,
                ":ip_src": pkt.src_ip,
                ":ip_dst": pkt.dst_ip,
                ":sport": pkt.sport,
                ":dport": pkt.dport,
                ":file_ptr": pkt.pkt_ptr,
                ":file_id": pkt.file_no,
                ":timestamp": pkt.timestamp,
            })
            .unwrap();
        }
        self.conn.execute("END TRANSACTION", []).unwrap();
    }

    pub fn new(db_filename: &String) -> Self {
        Database {
            filename: db_filename.to_string(),
            conn: Connection::open(db_filename).unwrap(),
        }
    }

    pub fn init(&self) {
        self.conn
            .execute(
                "create table if not exists packet (
            id integer not null primary key, 
            ip_src integer,
            ip_dst integer, 
            mac_src integer, 
            mac_dst integer, 
            sport integer, 
            dport integer, 
            file_ptr integer, 
            file_id integer,
            timestamp timestamp)",
                [],
            )
            .unwrap();
// CREATE INDEX index_name
// ON table_name (column_name);
        self.conn
            .execute(
                "create index if not exists by_timestamp on packet (timestamp);",
                [],
            )
            .unwrap();
        // let mut prep_insert = conn
        // .prepare_cached("insert into packet (sip,dip) values(?, ?)")
        // .unwrap();
        self.conn
            .execute_batch(
                "PRAGMA journal_mode = MEMORY;
                    PRAGMA synchronous = OFF;
                    PRAGMA cache_size = 1000000;
                    PRAGMA temp_store = MEMORY;
                    PRAGMA threads=4;"
                    // PRAGMA locking_mode = EXCLUSIVE;",
            )
            .expect("PRAGMA");
    }
}
