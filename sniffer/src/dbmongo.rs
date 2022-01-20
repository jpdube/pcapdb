use mongodb::{
    bson::doc, options::InsertManyOptions, sync::Client, sync::Collection, sync::Database,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

pub struct MDatabase {
    conn: Client,
    db: Database,
    collection: Collection<DbInfo>,
}

impl MDatabase {
    pub fn save_many(&self, pkt: &Vec<DbInfo>) {
        let mut insert_opts: InsertManyOptions = InsertManyOptions::default();
        insert_opts.ordered = Some(false);
        insert_opts.bypass_document_validation = Some(false);
        // let insert_opts = InsertManyOptions {
        // ordered: Some(false),
        // bypass_document_validation: Some(false),
        // write_concern: Some(None)
        // };
        self.collection.insert_many(pkt, insert_opts).unwrap();
    }

    pub fn save_db(&self, pkt: &DbInfo) {
        self.collection.insert_one(pkt, None).unwrap();
    }

    pub fn new() -> Self {
        let uri = "mongodb://localhost:27017/";
        let client = Client::with_uri_str(&uri).unwrap();
        let database = client.database("packet_sniffer");
        let coll = database.collection::<DbInfo>("packets");

        Self {
            conn: client,
            db: database,
            collection: coll,
        }
    }

    pub fn init(&self) {
        for collection_name in self.db.list_collection_names(None).unwrap() {
            println!("MONGODB collections list:{}", collection_name);
        }
    }
}
