use crate::dbmongo::{DbInfo, MDatabase};
use pcap::{Capture, Device};
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::SystemTime;
// use crate::db::{Database, DbInfo};
use crate::db_config::DBConfig;
use crate::packetref::PacketRef;

// const BUFFER_SIZE: usize = 32;
const MAX_FILE_SIZE: u64 = 20_000_000;
const PCAP_PATH: &str = "./db/pcap/";

const GLOBAL_HDR: [u8; 24] = [
    0xa1, 0xb2, 0xc3, 0xd4, 0x00, 0x02, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01,
];

pub fn capture(device_name: &String) -> Result<(), pcap::Error> {
    println!("Capture device: {}", device_name);
    let list: Vec<Device> = pcap::Device::list()?;
    let (tx_packet, rx_packet) = mpsc::channel();
    let (tx_db, rx_db) = mpsc::channel();

    //--- DB Thread
    let mut dbinfo_list: Vec<DbInfo> = Vec::new();

    thread::spawn(move || {
        let database: MDatabase = MDatabase::new();
        database.init();

        for p in rx_db {
            let pkt: DbInfo = p;
            dbinfo_list.push(pkt);

            if dbinfo_list.len() == 48 {
                let t_init = SystemTime::now();
                database.save_many(&dbinfo_list);
                dbinfo_list.clear();
                println!(
                    "DB Execution time: {}us",
                    t_init.elapsed().unwrap().as_micros()
                );
            }
        }
    });

    thread::spawn(move || {
        let mut file_ptr: u64 = 0;
        let mut file_no: u32;
        let mut dbconfig: DBConfig = DBConfig::new();
        file_no = dbconfig.read();

        let mut bin_file = File::create(format!("{}{}.pcap", PCAP_PATH, file_no)).unwrap();
        for p in rx_packet {
            if file_ptr >= MAX_FILE_SIZE {
                file_no = dbconfig.read();
                file_ptr = 0;
                bin_file = File::create(format!("{}{}.pcap", PCAP_PATH, file_no)).unwrap();
            }

            if file_ptr == 0 {
                bin_file.write_all(&GLOBAL_HDR).unwrap();
            }
            let mut pkt: PacketRef = p;
            file_ptr = bin_file.stream_position().unwrap();
            let db_pkt = DbInfo {
                src_mac: pkt.src_mac(),
                dst_mac: pkt.dst_mac(),
                ether_type: pkt.ether_type(),
                ip_proto: pkt.ip_proto(),
                src_ip: pkt.src_ip(),
                dst_ip: pkt.dst_ip(),
                sport: pkt.sport(),
                dport: pkt.dport(),
                pkt_ptr: file_ptr,
                file_no: file_no,
                timestamp: pkt.ts_sec,
            };
            tx_db.send(db_pkt).unwrap();

            //--- TCP
            if pkt.ip_proto() == 0x06 {
                bin_file.write_all(&pkt.pkt_header(true)).unwrap();
                bin_file.write_all(&pkt.get_header()).unwrap();
            //--- UDP
            } else if pkt.ip_proto() == 0x11 {
                bin_file.write_all(&pkt.pkt_header(true)).unwrap();
                bin_file.write_all(&pkt.get_header()).unwrap();
            } else {
                bin_file.write_all(&pkt.pkt_header(false)).unwrap();
                bin_file.write_all(&pkt.raw_packet).unwrap();
            }
        }
    });

    for dev in list {
        println!("Device: {:?}", dev);
        if dev.name == *device_name {
            let mut cap = Capture::from_device(dev)
                .unwrap()
                .promisc(true)
                .snaplen(65535)
                .open()
                .unwrap();

            let mut total: usize = 0;
            while let Ok(packet) = cap.next() {
                let pkt = PacketRef {
                    raw_packet: packet.data.to_vec(),
                    inc_len: packet.header.len,
                    orig_len: packet.header.caplen,
                    ts_sec: packet.header.ts.tv_sec as u32,
                    ts_usec: packet.header.ts.tv_usec as u32,
                    header_only: true,
                };

                total += 1;

                // let t_init = SystemTime::now();

                tx_packet.send(pkt.clone()).unwrap();
                // println!(
                // "Capture execution time: {}us",
                // t_init.elapsed().unwrap().as_micros()
                // );
            }
        }
    }
    Ok(())
}
