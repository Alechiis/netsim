use serde::{Serialize, Deserialize};

/// Represents a simulated network packet (simplified Layer 2/3)
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Packet {
    // Layer 2 (Ethernet)
    pub src_mac: String,
    pub dst_mac: String,
    // VLAN (802.1Q)
    pub vlan_id: Option<u16>,
    
    // Layer 3 (IP)
    pub src_ip: String,
    pub dst_ip: String,
    pub ttl: u8,
    pub protocol: IpProtocol,
    
    // Payload
    pub payload: PacketPayload,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IpProtocol {
    ICMP = 1,
    TCP = 6,
    UDP = 17,
    OSPF = 89,
    Unknown = 255,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PacketPayload {
    Icmp(IcmpPacket),
    Raw(String), // For generic data or unsupported protocols
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IcmpPacket {
    pub message_type: IcmpType,
    pub code: u8,
    pub id: u16,
    pub sequence: u16,
    pub data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IcmpType {
    EchoReply = 0,
    DestinationUnreachable = 3,
    EchoRequest = 8,
    TimeExceeded = 11,
}

impl Packet {
    pub fn new_icmp_echo(src_mac: &str, dst_mac: &str, src_ip: &str, dst_ip: &str, seq: u16) -> Self {
        Packet {
            src_mac: src_mac.to_string(),
            dst_mac: dst_mac.to_string(),
            vlan_id: None,
            src_ip: src_ip.to_string(),
            dst_ip: dst_ip.to_string(),
            ttl: 64,
            protocol: IpProtocol::ICMP,
            payload: PacketPayload::Icmp(IcmpPacket {
                message_type: IcmpType::EchoRequest,
                code: 0,
                id: 1, // Simulated process ID
                sequence: seq,
                data: "NetSimPingData".to_string(),
            }),
        }
    }
}
