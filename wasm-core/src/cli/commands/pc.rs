use crate::types::cli::CommandResult;
use crate::types::network::NetworkDevice;

/// Handle PC/Host specific commands
pub fn handle_pc_commands(device: &mut NetworkDevice, cmd: &str) -> Option<CommandResult> {
    
    // Ping command
    if cmd.starts_with("ping ") {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() < 2 {
            return Some(CommandResult {
                success: false,
                output: "Usage: ping <ip-address>".to_string(),
                new_view: None, new_hostname: None, });
        }
        
        let target_ip = parts[1];
        
        if !is_valid_ipv4(target_ip) {
            return Some(CommandResult {
                success: false,
                output: format!("Error: Invalid IP address '{}'", target_ip),
                new_view: None, new_hostname: None, });
        }
        
        // Simulate ping output
        let output = generate_ping_output(device, target_ip);
        return Some(CommandResult {
            success: true,
            output,
            new_view: None, new_hostname: None, });
    }
    
    // Traceroute command
    if cmd.starts_with("traceroute ") || cmd.starts_with("tracert ") {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() < 2 {
            return Some(CommandResult {
                success: false,
                output: "Usage: traceroute <ip-address>".to_string(),
                new_view: None, new_hostname: None, });
        }
        
        let target_ip = parts[1];
        
        if !is_valid_ipv4(target_ip) {
            return Some(CommandResult {
                success: false,
                output: format!("Error: Invalid IP address '{}'", target_ip),
                new_view: None, new_hostname: None, });
        }
        
        let output = generate_traceroute_output(target_ip);
        return Some(CommandResult {
            success: true,
            output,
            new_view: None, new_hostname: None, });
    }
    
    // IP configuration (PC style)
    if cmd.starts_with("ip ") && !cmd.starts_with("ip route") && !cmd.starts_with("ip address") {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        // Format: ip <address> <mask> <gateway>
        if parts.len() >= 4 {
            let ip = parts[1];
            let mask = parts[2];
            let gateway = parts[3];
            
            if !is_valid_ipv4(ip) || !is_valid_ipv4(gateway) {
                return Some(CommandResult {
                    success: false,
                    output: "Error: Invalid IP address format".to_string(),
                    new_view: None, new_hostname: None, });
            }
            
            // Apply to first port
            if let Some(port) = device.ports.first_mut() {
                port.config.ip_address = Some(ip.to_string());
                let cidr = if mask.contains('.') { mask_to_cidr(mask) } else { mask.parse().unwrap_or(24) };
                port.config.subnet_mask = Some(cidr);
            }
            
            return Some(CommandResult {
                success: true,
                output: format!("IP configuration set:\n  Address: {}\n  Mask: {}\n  Gateway: {}", ip, mask, gateway),
                new_view: None, new_hostname: None, });
        } else if parts.len() == 2 && parts[1] == "dhcp" {
            // ip dhcp
            return Some(CommandResult {
                success: true,
                output: "DHCP request sent...\nReceived IP: 192.168.1.100/24\nGateway: 192.168.1.1\nDNS: 8.8.8.8".to_string(),
                new_view: None, new_hostname: None, });
        }
    }
    
    // ipconfig / ifconfig
    if cmd == "ipconfig" || cmd == "ifconfig" || cmd == "ip a" || cmd == "ip addr" {
        let output = generate_ipconfig_output(device);
        return Some(CommandResult {
            success: true,
            output,
            new_view: None, new_hostname: None, });
    }
    
    // ipconfig /all
    if cmd == "ipconfig /all" || cmd == "ifconfig -a" {
        let output = generate_ipconfig_detailed(device);
        return Some(CommandResult {
            success: true,
            output,
            new_view: None, new_hostname: None, });
    }
    
    // ARP table
    if cmd == "arp -a" || cmd == "display arp" || cmd == "show arp" {
        let output = generate_arp_table();
        return Some(CommandResult {
            success: true,
            output,
            new_view: None, new_hostname: None, });
    }
    
    // DNS lookup
    if cmd.starts_with("nslookup ") {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() >= 2 {
            let domain = parts[1];
            return Some(CommandResult {
                success: true,
                output: format!(
                    "Server:  dns.netsim.local\nAddress: 8.8.8.8\n\nNon-authoritative answer:\nName:    {}\nAddress: 93.184.216.34 (simulated)",
                    domain
                ),
                new_view: None, new_hostname: None, });
        }
    }
    
    // Netstat
    if cmd == "netstat" || cmd == "netstat -an" {
        return Some(CommandResult {
            success: true,
            output: format!(
                "Active Connections\n\n\
                 Proto  Local Address          Foreign Address        State\n\
                 TCP    0.0.0.0:22             0.0.0.0:0              LISTENING\n\
                 TCP    0.0.0.0:80             0.0.0.0:0              LISTENING\n\
                 TCP    127.0.0.1:8080         127.0.0.1:49152        ESTABLISHED"
            ),
            new_view: None, new_hostname: None, });
    }
    
    None
}

use crate::simulation::{SIMULATION_ENGINE, Event};
use crate::types::packet::{Packet, IpProtocol, PacketPayload, IcmpPacket, IcmpType};

/// Generate ping output simulation using Packet Engine
fn generate_ping_output(device: &NetworkDevice, target_ip: &str) -> String {
    let mut lines = Vec::new();
    
    lines.push(format!("PING {} ({}) 56 bytes of data.", target_ip, target_ip));
    
    // Find source IP (first configured interface)
    let src_ip = device.ports.iter()
        .find_map(|p| p.config.ip_address.clone());
        
    let has_ip = src_ip.is_some();
    
    if !has_ip {
        lines.push(format!("From {}: Network is unreachable (no IP configured)", device.hostname));
        lines.push("".to_string());
        lines.push(format!("--- {} ping statistics ---", target_ip));
        lines.push("4 packets transmitted, 0 received, 100% packet loss".to_string());
        return lines.join("\n");
    }

    let src_ip_str = src_ip.unwrap();
    let mut received = 0;
    
    // Send 4 packets
    for seq in 1..=4 {
        // Create Echo Request
        let packet = Packet {
            src_mac: "00:00:00:00:00:01".to_string(), // Placeholder MAC
            dst_mac: "FF:FF:FF:FF:FF:FF".to_string(), // Broadcast/ARP needed in real calc
            vlan_id: None,
            src_ip: src_ip_str.clone(),
            dst_ip: target_ip.to_string(),
            ttl: 64,
            protocol: IpProtocol::ICMP,
            payload: PacketPayload::Icmp(IcmpPacket {
                message_type: IcmpType::EchoRequest,
                code: 0,
                id: 1,
                sequence: seq,
                data: "NetSimPingData".to_string(),
            }),
        };
        
        // Ask engine to process
        let mut engine = SIMULATION_ENGINE.lock().unwrap();
        
        // In simulation mode, we check immediate connectivity
        match engine.process_packet_immediate(&device.id, packet) {
            Some(reply) => {
                received += 1;
                // Simulate RTT variance
                let rtt = 1 + (seq % 3);
                lines.push(format!("64 bytes from {}: icmp_seq={} ttl={} time={}ms", 
                    reply.src_ip, seq, reply.ttl, rtt));
            },
            None => {
                lines.push(format!("Request timeout for icmp_seq={}", seq));
            }
        }
    }
    
    lines.push("".to_string());
    lines.push(format!("--- {} ping statistics ---", target_ip));
    let loss = if received == 4 { 0 } else { (4 - received) * 25 }; 
    lines.push(format!("4 packets transmitted, {} received, {}% packet loss", received, loss));
    
    lines.join("\n")
}

/// Generate traceroute output
fn generate_traceroute_output(target: &str) -> String {
    let mut lines = Vec::new();
    
    lines.push(format!("traceroute to {} ({}), 30 hops max, 60 byte packets", target, target));
    lines.push(" 1  gateway (192.168.1.1)  1.234 ms  1.123 ms  1.456 ms".to_string());
    lines.push(" 2  10.0.0.1  5.234 ms  5.123 ms  5.456 ms".to_string());
    lines.push(" 3  * * *".to_string());
    lines.push(format!(" 4  {} ({})  10.234 ms  10.123 ms  10.456 ms", target, target));
    
    lines.join("\n")
}

/// Generate ipconfig output
fn generate_ipconfig_output(device: &NetworkDevice) -> String {
    let mut lines = Vec::new();
    
    lines.push(format!("Host: {}", device.hostname));
    lines.push("".to_string());
    
    for port in &device.ports {
        lines.push(format!("Interface: {}", port.name));
        
        if let Some(ip) = &port.config.ip_address {
            let mask = port.config.subnet_mask.unwrap_or(24);
            lines.push(format!("   IPv4 Address: {}", ip));
            lines.push(format!("   Subnet Mask:  {}", cidr_to_mask(mask)));
        } else {
            lines.push("   IPv4 Address: (not configured)".to_string());
        }
        lines.push("".to_string());
    }
    
    lines.join("\n")
}

/// Generate detailed ipconfig output
fn generate_ipconfig_detailed(device: &NetworkDevice) -> String {
    let mut lines = Vec::new();
    
    lines.push(format!("Host Name: {}", device.hostname));
    lines.push(format!("Primary DNS Suffix: netsim.local"));
    lines.push("Node Type: Hybrid".to_string());
    lines.push("IP Routing Enabled: Yes".to_string());
    lines.push("".to_string());
    
    for port in &device.ports {
        lines.push(format!("Ethernet adapter {}:", port.name));
        lines.push("".to_string());
        lines.push("   Connection-specific DNS Suffix: netsim.local".to_string());
        lines.push("   Description: NetSim Virtual NIC".to_string());
        lines.push("   Physical Address: 00-50-56-C0-00-01".to_string());
        lines.push(format!("   DHCP Enabled: {}", if port.config.mode == crate::types::network::PortMode::Routed { "No" } else { "Yes" }));
        
        if let Some(ip) = &port.config.ip_address {
            let mask = port.config.subnet_mask.unwrap_or(24);
            lines.push(format!("   IPv4 Address: {}", ip));
            lines.push(format!("   Subnet Mask: {}", cidr_to_mask(mask)));
            lines.push("   Default Gateway: 192.168.1.1".to_string());
        } else {
            lines.push("   Media State: Media disconnected".to_string());
        }
        lines.push("".to_string());
    }
    
    lines.join("\n")
}

/// Generate ARP table
fn generate_arp_table() -> String {
    format!(
        "Address Resolution Protocol\n\
         \n\
         Interface: eth0 (192.168.1.100)\n\
         Internet Address      Physical Address       Type\n\
         192.168.1.1           00-50-56-c0-00-08      dynamic\n\
         192.168.1.254         00-50-56-c0-00-01      dynamic\n\
         224.0.0.22            01-00-5e-00-00-16      static"
    )
}

/// Validate IPv4 address format
fn is_valid_ipv4(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    for part in parts {
        match part.parse::<u8>() {
            Ok(_) => continue,
            Err(_) => return false,
        }
    }
    true
}

/// Convert CIDR to dotted decimal mask
fn cidr_to_mask(cidr: u8) -> String {
    if cidr > 32 {
        return "255.255.255.255".to_string();
    }
    let mask: u32 = if cidr == 0 { 0 } else { !0u32 << (32 - cidr) };
    format!("{}.{}.{}.{}", 
        (mask >> 24) & 0xFF,
        (mask >> 16) & 0xFF,
        (mask >> 8) & 0xFF,
        mask & 0xFF
    )
}

/// Convert dotted decimal mask to CIDR
fn mask_to_cidr(mask: &str) -> u8 {
    let parts: Vec<u8> = mask.split('.')
        .filter_map(|p| p.parse().ok())
        .collect();
    if parts.len() != 4 {
        return 24;
    }
    let mask_int: u32 = ((parts[0] as u32) << 24) 
                      | ((parts[1] as u32) << 16) 
                      | ((parts[2] as u32) << 8) 
                      | (parts[3] as u32);
    mask_int.count_ones() as u8
}

