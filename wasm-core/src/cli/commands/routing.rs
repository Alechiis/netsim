use crate::types::cli::{CliView, CommandResult};
use crate::types::network::NetworkDevice;

/// Helper to create error for wrong view
fn view_error() -> CommandResult {
    CommandResult {
        success: false,
        output: "Error: Command requires system-view. Enter 'system-view' first.".to_string(),
        new_view: None,
        new_hostname: None,
    }
}

/// Handle routing-related commands
pub fn handle_routing_commands(device: &mut NetworkDevice, cmd: &str, current_view: &CliView) -> Option<CommandResult> {
    
    // ========== Display commands - allowed from any view ==========
    
    if cmd == "display ip routing-table" || cmd == "show ip route" || cmd == "show ip route static" {
        let output = generate_routing_table(device);
        return Some(CommandResult {
            success: true,
            output,
            new_view: None,
            new_hostname: None,
        });
    }
    
    if cmd == "display ospf neighbor" || cmd == "show ip ospf neighbor" {
        let output = generate_ospf_neighbor_table(device);
        return Some(CommandResult {
            success: true,
            output,
            new_view: None,
            new_hostname: None,
        });
    }
    
    if cmd == "display ospf lsdb" || cmd == "show ip ospf database" {
        return Some(CommandResult {
            success: true,
            output: "OSPF Link State Database - Area 0\n\n(No entries - simulation mode)".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }
    
    if cmd == "display bgp peer" || cmd == "show ip bgp summary" || cmd == "show bgp summary" {
        let output = generate_bgp_summary(device);
        return Some(CommandResult {
            success: true,
            output,
            new_view: None,
            new_hostname: None,
        });
    }
    
    if cmd == "display bgp routing-table" || cmd == "show ip bgp" {
        return Some(CommandResult {
            success: true,
            output: "BGP Routing Table\n\n(No routes - simulation mode)".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }
    
    // ========== Configuration commands - require system-view ==========
    
    // Static route
    if cmd.starts_with("ip route-static ") || cmd.starts_with("ip route ") {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() < 5 {
            return Some(CommandResult {
                success: false,
                output: "Error: Incomplete command. Usage: ip route-static <dest> <mask> <next-hop>".to_string(),
                new_view: None,
                new_hostname: None,
            });
        }
        
        let dest = parts[2];
        let mask_str = parts[3];
        let next_hop = parts[4];
        
        if !is_valid_ipv4(dest) {
            return Some(CommandResult {
                success: false,
                output: format!("Error: Invalid destination address '{}'", dest),
                new_view: None,
                new_hostname: None,
            });
        }
        
        if !is_valid_ipv4(next_hop) {
            return Some(CommandResult {
                success: false,
                output: format!("Error: Invalid next-hop address '{}'", next_hop),
                new_view: None,
                new_hostname: None,
            });
        }
        
        let mask = if mask_str.contains('.') {
            mask_to_cidr(mask_str)
        } else {
            mask_str.parse::<u8>().unwrap_or(24)
        };
        
        return Some(CommandResult {
            success: true,
            output: format!("Static route added: {}/{} via {}", dest, mask, next_hop),
            new_view: None,
            new_hostname: None,
        });
    }
    
    // Delete static route
    if cmd.starts_with("undo ip route-static ") || cmd.starts_with("no ip route ") {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() >= 4 {
            let dest = parts[3];
            return Some(CommandResult {
                success: true,
                output: format!("Static route to {} removed", dest),
                new_view: None,
                new_hostname: None,
            });
        }
        return Some(CommandResult {
            success: false,
            output: "Error: Incomplete command".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }
    
    // Enable OSPF
    if cmd.starts_with("ospf ") || cmd.starts_with("router ospf ") {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let process_id = parts.last().unwrap_or(&"1");
        device.ospf_enabled = Some(true);
        
        return Some(CommandResult {
            success: true,
            output: format!("OSPF process {} enabled", process_id),
            new_view: None,
            new_hostname: None,
        });
    }
    
    // Disable OSPF
    if cmd == "undo ospf" || cmd == "no router ospf" {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        device.ospf_enabled = Some(false);
        return Some(CommandResult {
            success: true,
            output: "OSPF disabled".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }
    
    // OSPF area command
    if cmd.starts_with("area ") {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() >= 2 {
            let area_id = parts[1];
            return Some(CommandResult {
                success: true,
                output: format!("Entered OSPF area {} configuration", area_id),
                new_view: None,
                new_hostname: None,
            });
        }
    }
    
    // OSPF network command
    if cmd.starts_with("network ") && cmd.contains("area") {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() >= 5 {
            let network = parts[1];
            let area = parts.last().unwrap_or(&"0");
            return Some(CommandResult {
                success: true,
                output: format!("Network {} added to OSPF area {}", network, area),
                new_view: None,
                new_hostname: None,
            });
        }
    } else if cmd.starts_with("network ") {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() >= 2 {
            let network = parts[1];
            return Some(CommandResult {
                success: true,
                output: format!("Network {} added to OSPF", network),
                new_view: None,
                new_hostname: None,
            });
        }
    }
    
    // Enable BGP
    if cmd.starts_with("bgp ") || cmd.starts_with("router bgp ") {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let as_number = parts.last().unwrap_or(&"65000");
        device.bgp_enabled = Some(true);
        
        return Some(CommandResult {
            success: true,
            output: format!("BGP AS {} enabled", as_number),
            new_view: None,
            new_hostname: None,
        });
    }
    
    // Disable BGP
    if cmd == "undo bgp" || cmd.starts_with("no router bgp") {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        device.bgp_enabled = Some(false);
        return Some(CommandResult {
            success: true,
            output: "BGP disabled".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }
    
    // BGP peer/neighbor
    if cmd.starts_with("peer ") || cmd.starts_with("neighbor ") {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() >= 4 {
            let peer_ip = parts[1];
            let as_idx = parts.iter().position(|&p| p == "as-number" || p == "remote-as");
            if let Some(idx) = as_idx {
                if parts.len() > idx + 1 {
                    let remote_as = parts[idx + 1];
                    return Some(CommandResult {
                        success: true,
                        output: format!("BGP peer {} AS {} configured", peer_ip, remote_as),
                        new_view: None,
                        new_hostname: None,
                    });
                }
            }
        }
        return Some(CommandResult {
            success: false,
            output: "Error: Incomplete peer command. Usage: peer <ip> as-number <as>".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }
    
    None
}

/// Generate routing table display
fn generate_routing_table(device: &NetworkDevice) -> String {
    let mut lines = Vec::new();
    lines.push("Routing Table: Main".to_string());
    lines.push(format!("Device: {}", device.hostname));
    lines.push("".to_string());
    lines.push(format!("{:<20} {:<8} {:<6} {:<6} {:<16} {:<15}", 
        "Destination/Mask", "Proto", "Pre", "Cost", "NextHop", "Interface"));
    lines.push("-".repeat(75));
    
    for port in &device.ports {
        if let Some(ip) = &port.config.ip_address {
            let mask = port.config.subnet_mask.unwrap_or(24);
            let network = calculate_network(ip, mask);
            lines.push(format!("{:<20} {:<8} {:<6} {:<6} {:<16} {:<15}",
                format!("{}/{}", network, mask), "Direct", "0", "0", "127.0.0.1", &port.name));
        }
    }
    
    lines.push("".to_string());
    lines.push("(Additional routes would appear here in full implementation)".to_string());
    lines.join("\n")
}

/// Generate OSPF neighbor table
fn generate_ospf_neighbor_table(device: &NetworkDevice) -> String {
    if device.ospf_enabled != Some(true) {
        return "OSPF is not enabled on this device.".to_string();
    }
    "OSPF Neighbor Information\n\n(No OSPF neighbors discovered - connect routers to establish adjacencies)".to_string()
}

/// Generate BGP summary
fn generate_bgp_summary(device: &NetworkDevice) -> String {
    if device.bgp_enabled != Some(true) {
        return "BGP is not enabled on this device.".to_string();
    }
    format!("BGP Summary\nRouter ID: {} (simulated)\nLocal AS: 65000 (simulated)\n\n(No BGP peers configured)", device.hostname)
}

fn is_valid_ipv4(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 { return false; }
    parts.iter().all(|p| p.parse::<u8>().is_ok())
}

fn mask_to_cidr(mask: &str) -> u8 {
    let parts: Vec<u8> = mask.split('.').filter_map(|p| p.parse().ok()).collect();
    if parts.len() != 4 { return 24; }
    let mask_int: u32 = ((parts[0] as u32) << 24) | ((parts[1] as u32) << 16) | ((parts[2] as u32) << 8) | (parts[3] as u32);
    mask_int.count_ones() as u8
}

fn calculate_network(ip: &str, cidr: u8) -> String {
    let parts: Vec<u8> = ip.split('.').filter_map(|p| p.parse().ok()).collect();
    if parts.len() != 4 { return ip.to_string(); }
    let ip_int: u32 = ((parts[0] as u32) << 24) | ((parts[1] as u32) << 16) | ((parts[2] as u32) << 8) | (parts[3] as u32);
    let mask: u32 = if cidr >= 32 { !0u32 } else if cidr == 0 { 0 } else { !0u32 << (32 - cidr) };
    let network = ip_int & mask;
    format!("{}.{}.{}.{}", (network >> 24) & 0xFF, (network >> 16) & 0xFF, (network >> 8) & 0xFF, network & 0xFF)
}
