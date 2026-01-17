use crate::types::cli::{CliView, CommandResult};
use crate::types::network::{NetworkDevice, PortMode, LinkStatus, NetworkPort};

/// Helper to create error for wrong view
fn view_error(required: &str) -> CommandResult {
    CommandResult {
        success: false,
        output: format!("Error: Command requires {}. Enter '{}' first.", required, 
            if required == "system-view" { "system-view" } else { "interface <name>" }),
        new_view: None,
        new_hostname: None,
    }
}

/// Handle interface-related commands
pub fn handle_interface_commands(device: &mut NetworkDevice, cmd: &str, current_view: &CliView) -> Option<CommandResult> {
    
    // Enter interface view - requires system-view
    if cmd.starts_with("interface ") {
        if *current_view != CliView::SystemView && *current_view != CliView::InterfaceView {
            return Some(view_error("system-view"));
        }
        
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() < 2 {
            return Some(CommandResult {
                success: false,
                output: "Incomplete command. Usage: interface <type><number>".to_string(),
                new_view: None,
                new_hostname: None,
            });
        }
        
        let port_name = parts[1..].join("");
        if let Some(_port) = device.ports.iter().find(|p| p.name.eq_ignore_ascii_case(&port_name)) {
            return Some(CommandResult {
                success: true,
                output: format!("Entered interface view for {}", port_name),
                new_view: Some(CliView::InterfaceView),
                new_hostname: None,
            });
        } else {
            return Some(CommandResult {
                success: false,
                output: format!("Error: Interface {} not found.", port_name),
                new_view: None,
                new_hostname: None,
            });
        }
    }

    // Display commands - allowed from any view (show/display are read-only)
    if cmd == "display ip interface brief" || cmd == "show ip interface brief" {
        let output = generate_interface_brief(device);
        return Some(CommandResult {
            success: true,
            output,
            new_view: None,
            new_hostname: None,
        });
    }

    if cmd.starts_with("display interface") || cmd.starts_with("show interface") {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() >= 3 {
            let iface_name = parts[2..].join("");
            if let Some(port) = device.ports.iter().find(|p| p.name.eq_ignore_ascii_case(&iface_name)) {
                return Some(CommandResult {
                    success: true,
                    output: generate_interface_detail(port),
                    new_view: None,
                    new_hostname: None,
                });
            } else {
                return Some(CommandResult {
                    success: false,
                    output: format!("Error: Interface {} not found.", iface_name),
                    new_view: None,
                    new_hostname: None,
                });
            }
        } else {
            let mut output = String::new();
            for port in &device.ports {
                output.push_str(&generate_interface_detail(port));
                output.push_str("\n");
            }
            return Some(CommandResult {
                success: true,
                output,
                new_view: None,
                new_hostname: None,
            });
        }
    }

    // ========== Commands that require interface-view ==========

    // IP Address configuration - requires interface-view
    if cmd.starts_with("ip address ") {
        if *current_view != CliView::InterfaceView {
            return Some(view_error("interface-view"));
        }
        
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() < 4 {
            return Some(CommandResult {
                success: false,
                output: "Error: Incomplete command. Usage: ip address <ip> <mask>".to_string(),
                new_view: None,
                new_hostname: None,
            });
        }
        let ip = parts[2];
        let mask_str = parts[3];
        
        if !is_valid_ipv4(ip) {
            return Some(CommandResult {
                success: false,
                output: format!("Error: Invalid IP address '{}'", ip),
                new_view: None,
                new_hostname: None,
            });
        }
        
        let mask = if mask_str.contains('.') {
            mask_to_cidr(mask_str)
        } else {
            mask_str.parse::<u8>().unwrap_or(24)
        };
        
        // Apply to first routed port (placeholder - should use tracked current interface)
        if let Some(port) = device.ports.iter_mut().find(|p| p.config.mode == PortMode::Routed) {
            port.config.ip_address = Some(ip.to_string());
            port.config.subnet_mask = Some(mask);
            return Some(CommandResult {
                success: true,
                output: format!("IP address {} {} configured", ip, mask_str),
                new_view: None,
                new_hostname: None,
            });
        }
        
        return Some(CommandResult {
            success: false,
            output: "Error: No routed interface available".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }

    // Undo IP address - requires interface-view
    if cmd == "undo ip address" || cmd == "no ip address" {
        if *current_view != CliView::InterfaceView {
            return Some(view_error("interface-view"));
        }
        
        if let Some(port) = device.ports.iter_mut().find(|p| p.config.ip_address.is_some()) {
            port.config.ip_address = None;
            port.config.subnet_mask = None;
            return Some(CommandResult {
                success: true,
                output: "IP address removed".to_string(),
                new_view: None,
                new_hostname: None,
            });
        }
        return Some(CommandResult {
            success: false,
            output: "Error: No IP address to remove".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }

    // Shutdown - requires interface-view
    if cmd == "shutdown" {
        if *current_view != CliView::InterfaceView {
            return Some(view_error("interface-view"));
        }
        return Some(CommandResult {
            success: true,
            output: "Interface administratively disabled".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }

    // Undo shutdown / no shutdown - requires interface-view
    if cmd == "undo shutdown" || cmd == "no shutdown" {
        if *current_view != CliView::InterfaceView {
            return Some(view_error("interface-view"));
        }
        return Some(CommandResult {
            success: true,
            output: "Interface enabled".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }

    // Description - requires interface-view
    if cmd.starts_with("description ") {
        if *current_view != CliView::InterfaceView {
            return Some(view_error("interface-view"));
        }
        
        let desc = cmd.strip_prefix("description ").unwrap_or("").trim();
        if desc.is_empty() {
            return Some(CommandResult {
                success: false,
                output: "Error: Description cannot be empty".to_string(),
                new_view: None,
                new_hostname: None,
            });
        }
        return Some(CommandResult {
            success: true,
            output: format!("Description set to '{}'", desc),
            new_view: None,
            new_hostname: None,
        });
    }

    // Port link-type - requires interface-view (switch)
    if cmd.starts_with("port link-type ") || cmd.starts_with("switchport mode ") {
        if *current_view != CliView::InterfaceView {
            return Some(view_error("interface-view"));
        }
        
        let mode_str = cmd.split_whitespace().last().unwrap_or("access");
        let mode = match mode_str {
            "access" => PortMode::Access,
            "trunk" => PortMode::Trunk,
            "hybrid" => PortMode::Hybrid,
            _ => {
                return Some(CommandResult {
                    success: false,
                    output: format!("Error: Invalid port mode '{}'. Use access, trunk, or hybrid.", mode_str),
                    new_view: None,
                    new_hostname: None,
                });
            }
        };
        
        return Some(CommandResult {
            success: true,
            output: format!("Port link-type set to {}", mode_str),
            new_view: None,
            new_hostname: None,
        });
    }

    // Port default VLAN - requires interface-view
    if cmd.starts_with("port default vlan ") || cmd.starts_with("switchport access vlan ") {
        if *current_view != CliView::InterfaceView {
            return Some(view_error("interface-view"));
        }
        
        let vlan_str = cmd.split_whitespace().last().unwrap_or("1");
        match vlan_str.parse::<u16>() {
            Ok(vlan) if vlan >= 1 && vlan <= 4094 => {
                return Some(CommandResult {
                    success: true,
                    output: format!("Access VLAN set to {}", vlan),
                    new_view: None,
                    new_hostname: None,
                });
            }
            _ => {
                return Some(CommandResult {
                    success: false,
                    output: "Error: Invalid VLAN ID (1-4094)".to_string(),
                    new_view: None,
                    new_hostname: None,
                });
            }
        }
    }

    // Trunk allowed VLANs - requires interface-view
    if cmd.starts_with("port trunk allow-pass vlan ") || cmd.starts_with("switchport trunk allowed vlan ") {
        if *current_view != CliView::InterfaceView {
            return Some(view_error("interface-view"));
        }
        
        let vlan_spec = cmd.split("vlan ").last().unwrap_or("1");
        return Some(CommandResult {
            success: true,
            output: format!("Trunk allowed VLANs set to: {}", vlan_spec),
            new_view: None,
            new_hostname: None,
        });
    }

    // Speed configuration - requires interface-view
    if cmd.starts_with("speed ") {
        if *current_view != CliView::InterfaceView {
            return Some(view_error("interface-view"));
        }
        
        let speed = cmd.strip_prefix("speed ").unwrap_or("auto").trim();
        return Some(CommandResult {
            success: true,
            output: format!("Speed set to {}", speed),
            new_view: None,
            new_hostname: None,
        });
    }

    // Duplex configuration - requires interface-view
    if cmd.starts_with("duplex ") {
        if *current_view != CliView::InterfaceView {
            return Some(view_error("interface-view"));
        }
        
        let duplex = cmd.strip_prefix("duplex ").unwrap_or("auto").trim();
        return Some(CommandResult {
            success: true,
            output: format!("Duplex set to {}", duplex),
            new_view: None,
            new_hostname: None,
        });
    }

    None
}

/// Generate brief interface summary
fn generate_interface_brief(device: &NetworkDevice) -> String {
    let mut lines = Vec::new();
    lines.push(format!("{:<20} {:<16} {:<8} {:<12} {:<10}", 
        "Interface", "IP-Address", "Status", "Protocol", "Mode"));
    lines.push("-".repeat(70));
    
    for port in &device.ports {
        let ip = port.config.ip_address.as_ref()
            .map(|ip| format!("{}/{}", ip, port.config.subnet_mask.unwrap_or(24)))
            .unwrap_or_else(|| "unassigned".to_string());
        
        let status = match port.status {
            LinkStatus::Up => "up",
            LinkStatus::Down => "down",
        };
        
        let protocol = if port.config.enabled { "up" } else { "down" };
        
        let mode = match port.config.mode {
            PortMode::Access => "access",
            PortMode::Trunk => "trunk",
            PortMode::Hybrid => "hybrid",
            PortMode::Routed => "routed",
        };
        
        lines.push(format!("{:<20} {:<16} {:<8} {:<12} {:<10}",
            port.name, ip, status, protocol, mode));
    }
    
    lines.join("\n")
}

/// Generate detailed interface information
fn generate_interface_detail(port: &NetworkPort) -> String {
    let mut lines = Vec::new();
    lines.push(format!("Interface: {}", port.name));
    lines.push(format!("  Type: {:?}", port.port_type));
    lines.push(format!("  Status: {:?}", port.status));
    lines.push(format!("  Admin Status: {}", if port.config.enabled { "up" } else { "administratively down" }));
    lines.push(format!("  Mode: {:?}", port.config.mode));
    
    if let Some(ip) = &port.config.ip_address {
        lines.push(format!("  IP Address: {}/{}", ip, port.config.subnet_mask.unwrap_or(24)));
    }
    
    if let Some(desc) = &port.config.description {
        lines.push(format!("  Description: {}", desc));
    }
    
    if let Some(vlan) = port.config.vlan {
        lines.push(format!("  VLAN: {}", vlan));
    }
    
    if let Some(vlans) = &port.config.allowed_vlans {
        let vlans_str: Vec<String> = vlans.iter().map(|v| v.to_string()).collect();
        lines.push(format!("  Allowed VLANs: {}", vlans_str.join(",")));
    }
    
    lines.join("\n")
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
