const IP_HDR_LEN_POS: usize = 0x0e;
const TCP_HDR_LEN_POS: usize = 0x2e;

const ETHERNET_HDR_LEN: usize = 0x0e;
const UDP_HEADER_LEN: u8 = 8;

const ETHER_IPV4_PROTO: u16 = 0x0800;
const ETHER_IPV6_PROTO: u16 = 0x08DD;
const ETHER_ARP_PROTO: u16 = 0x0806;

const IP_TCP_PROTO: u8 = 0x06;
const IP_UDP_PROTO: u8 = 0x11;
const IP_ICMP_PROTO: u8 = 0x01;



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PacketRef {
    pub ts_sec: u32,
    pub ts_usec: u32,
    pub inc_len: u32,
    pub orig_len: u32,
    pub raw_packet: Vec<u8>,
    pub header_only: bool
}

impl PacketRef {

    pub fn new () -> Self {
        Self {
            raw_packet: vec![0],
            inc_len: 0,
            orig_len: 0,
            ts_sec: 0,
            ts_usec: 0,
            header_only: true
        }         
    }

    pub fn pkt_header(&mut self, header_only: bool) -> [u8; 16] {
        let mut header: [u8; 16] = [0; 16];

        //--- Time second
        header[0] = ((self.ts_sec & 0xff000000) >> 24) as u8;
        header[1] = ((self.ts_sec & 0x00ff0000) >> 16) as u8;
        header[2] = ((self.ts_sec & 0x0000ff00) >> 8) as u8;
        header[3] = (self.ts_sec & 0x000000ff) as u8;

        //--- Time micro-second
        header[4] = ((self.ts_usec & 0xff000000) >> 24) as u8;
        header[5] = ((self.ts_usec & 0x00ff0000) >> 16) as u8;
        header[6] = ((self.ts_usec & 0x0000ff00) >> 8) as u8;
        header[7] = (self.ts_usec & 0x000000ff) as u8;

        //--- Included length
        if header_only {
            self.inc_len = self.get_header_len() as u32;
        }

        header[8] = ((self.inc_len & 0xff000000) >> 24) as u8;
        header[9] = ((self.inc_len & 0x00ff0000) >> 16) as u8;
        header[10] = ((self.inc_len & 0x0000ff00) >> 8) as u8;
        header[11] = (self.inc_len & 0x000000ff) as u8;

        //--- Actual length
        header[12] = ((self.orig_len & 0xff000000) >> 24) as u8;
        header[13] = ((self.orig_len & 0x00ff0000) >> 16) as u8;
        header[14] = ((self.orig_len & 0x0000ff00) >> 8) as u8;
        header[15] = (self.orig_len & 0x000000ff) as u8;

        return header;
    }

    pub fn dst_mac(&self) -> u64 {
        let mut mac_addr: u64;

        mac_addr = ((self.raw_packet[0] as u64) & 0x00000000000000ff) << 40;
        mac_addr += ((self.raw_packet[1] as u64) & 0x00000000000000ff) << 32;
        mac_addr += ((self.raw_packet[2] as u64) & 0x00000000000000ff) << 24;
        mac_addr += ((self.raw_packet[3] as u64) & 0x00000000000000ff) << 16;
        mac_addr += ((self.raw_packet[4] as u64) & 0x00000000000000ff) << 8;
        mac_addr += (self.raw_packet[5] as u64) & 0x00000000000000ff;

        mac_addr
    }

    pub fn src_mac(&self) -> u64 {
        let mut mac_addr: u64;

        mac_addr = ((self.raw_packet[6] as u64) & 0x00000000000000ff) << 40;
        mac_addr += ((self.raw_packet[7] as u64) & 0x00000000000000ff) << 32;
        mac_addr += ((self.raw_packet[8] as u64) & 0x00000000000000ff) << 24;
        mac_addr += ((self.raw_packet[9] as u64) & 0x00000000000000ff) << 16;
        mac_addr += ((self.raw_packet[10] as u64) & 0x00000000000000ff) << 8;
        mac_addr += (self.raw_packet[11] as u64) & 0x00000000000000ff;

        mac_addr
    }

    pub fn ether_type(&self) -> u16 {
        let mut etype: u16;

        etype = (self.raw_packet[12] as u16) << 8;
        etype += self.raw_packet[13] as u16;

        etype
    }

    pub fn src_ip(&self) -> u32 {
        let mut ip_addr: u32 = 0;

        if self.ether_type() == ETHER_IPV4_PROTO {
            ip_addr = (self.raw_packet[26] as u32) << 24;
            ip_addr += (self.raw_packet[27] as u32) << 16;
            ip_addr += (self.raw_packet[28] as u32) << 8;
            ip_addr += self.raw_packet[29] as u32;
        }

        ip_addr
    }

    pub fn dst_ip(&self) -> u32 {
        let mut ip_addr: u32 = 0;

        if self.ether_type() == ETHER_IPV4_PROTO {
            ip_addr = (self.raw_packet[30] as u32) << 24;
            ip_addr += (self.raw_packet[31] as u32) << 16;
            ip_addr += (self.raw_packet[32] as u32) << 8;
            ip_addr += self.raw_packet[33] as u32;
        }

        ip_addr
    }

    pub fn ip_proto(&self) -> u8 {
        let mut ip_proto: u8 = 0;

        if self.ether_type() == ETHER_IPV4_PROTO {
            ip_proto = self.raw_packet[23];
        }

        ip_proto
    }

    pub fn sport(&self) -> u16 {
        let mut port: u16 = 0;

        if self.ip_proto() == IP_UDP_PROTO || self.ip_proto() == IP_TCP_PROTO {
            port = (self.raw_packet[34] as u16) << 8;
            port += self.raw_packet[35] as u16;
        }

        port
    }

    pub fn dport(&self) -> u16 {
        let mut port: u16 = 0;

        if self.ip_proto() == IP_UDP_PROTO || self.ip_proto() == IP_TCP_PROTO {
            port = (self.raw_packet[36] as u16) << 8;
            port += self.raw_packet[37] as u16;
        }

        port
    }

    pub fn get_payload(&self) -> Vec<u8> {

        match self.ip_proto() {
            IP_TCP_PROTO => self.raw_packet[self.get_ip_header_len() as usize..].to_vec(),
            IP_UDP_PROTO => self.raw_packet[self.get_ip_header_len() as usize..].to_vec(),
            _ => self.raw_packet.to_vec()
        }
    }

    pub fn get_ip_header_len(&self) -> u8 {

        let hdr_len = self.raw_packet[IP_HDR_LEN_POS];

        (hdr_len >> 4) * 4
    }

    pub fn get_udp_header_len(&self) -> u8 {

        return UDP_HEADER_LEN
    }

    pub fn get_tcp_header_len(&self) -> u8 {

        (self.raw_packet[TCP_HDR_LEN_POS] >> 4) * 4
    }

    pub fn get_header_len(&self) -> u16 {

        //--- ethernet: 0x0d
        //--- ip header len: 0x0e
        //--- TCP header len position: 0x2e
        //--- UDP end of header: 0x29

        match self.ip_proto() {
            0x06 => (ETHERNET_HDR_LEN as u16 + self.get_ip_header_len() as u16 + self.get_tcp_header_len() as u16),
            0x11 => (ETHERNET_HDR_LEN as u16 + self.get_ip_header_len() as u16 + UDP_HEADER_LEN as u16) as u16,
            _ => self.inc_len as u16
        }

    }

    pub fn get_data_len(&self) -> u16 {
        self.raw_packet.len() as u16 - self.get_header_len()
    }

    pub fn get_header(&mut self) -> Vec<u8> {
        let hdr_len: u32 = self.get_header_len() as u32;

        self.inc_len = hdr_len;

        self.raw_packet[0..hdr_len as usize].to_vec()
    }
}
