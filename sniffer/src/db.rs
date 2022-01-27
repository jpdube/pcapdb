use rusqlite::{Connection, Result, Transaction};

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
        let sql = "INSERT INTO packet (mac_src, mac_dst, ip_src, ip_dst, sport, dport, file_ptr, file_id, timestamp) values (?,?,?,?,?,?,?,?,?)";

        let tx = self.conn.transaction().unwrap();

        for pkt in pkt_list.iter() {
            tx.execute(
                sql,
                [
                    pkt.src_mac,
                    pkt.dst_mac,
                    pkt.src_ip.into(),
                    pkt.dst_ip.into(),
                    pkt.sport.into(),
                    pkt.dport.into(),
                    pkt.pkt_ptr,
                    pkt.file_no.into(),
                    pkt.timestamp.into(),
                ],
            )
            .unwrap();
        }

        tx.commit().unwrap();
    }

    pub fn new(db_filename: &String) -> Self {
        Database {
            filename: db_filename.to_string(),
            conn: Connection::open(db_filename).unwrap(),
        }

    }

    pub fn init(&self) {

        self.conn.execute(
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

        // let mut prep_insert = conn
        // .prepare_cached("insert into packet (sip,dip) values(?, ?)")
        // .unwrap();
        self.conn.execute_batch(
            "PRAGMA journal_mode = OFF;
                    PRAGMA synchronous = 0;
                    PRAGMA cache_size = 1000000;
                    PRAGMA temp_store = MEMORY;
                    PRAGMA locking_mode = EXCLUSIVE;",
        )
        .expect("PRAGMA");

    }
}
