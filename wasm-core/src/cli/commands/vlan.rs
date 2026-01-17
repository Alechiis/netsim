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

/// Handle VLAN configuration commands
pub fn handle_vlan_commands(device: &mut NetworkDevice, cmd: &str, current_view: &CliView) -> Option<CommandResult> {
    
    // Display commands - allowed from any view (read-only)
    if cmd == "display vlan" || cmd == "show vlan" || cmd == "show vlan brief" {
        let output = generate_vlan_table(device);
        return Some(CommandResult {
            success: true,
            output,
            new_view: None,
            new_hostname: None,
        });
    }
    
    // Display specific VLAN - allowed from any view (read-only)
    if cmd.starts_with("display vlan ") || cmd.starts_with("show vlan id ") {
        let vlan_str = cmd.split_whitespace().last().unwrap_or("");
        return match vlan_str.parse::<u16>() {
            Ok(vlan) => {
                if device.vlans.contains(&vlan) || vlan == 1 {
                    Some(CommandResult {
                        success: true,
                        output: format!(
                            "VLAN {}\n  Name: VLAN{:04}\n  Status: active\n  Ports: (none assigned)",
                            vlan, vlan
                        ),
                        new_view: None,
                        new_hostname: None,
                    })
                } else {
                    Some(CommandResult {
                        success: false,
                        output: format!("Error: VLAN {} not found", vlan),
                        new_view: None,
                        new_hostname: None,
                    })
                }
            },
            _ => Some(CommandResult {
                success: false,
                output: "Error: Invalid VLAN ID".to_string(),
                new_view: None,
                new_hostname: None,
            }),
        };
    }
    
    // ========== Configuration commands - require system-view ==========
    
    // Create single VLAN
    if cmd.starts_with("vlan ") && !cmd.contains("batch") {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        let vlan_str = cmd.strip_prefix("vlan ").unwrap_or("").trim();
        return match vlan_str.parse::<u16>() {
            Ok(vlan) if vlan >= 1 && vlan <= 4094 => {
                if !device.vlans.contains(&vlan) {
                    device.vlans.push(vlan);
                    device.vlans.sort();
                }
                Some(CommandResult {
                    success: true,
                    output: format!("VLAN {} created", vlan),
                    new_view: None,
                    new_hostname: None,
                })
            },
            _ => Some(CommandResult {
                success: false,
                output: "Error: Invalid VLAN ID. Must be 1-4094.".to_string(),
                new_view: None,
                new_hostname: None,
            }),
        };
    }
    
    // Create VLAN batch (Huawei)
    if cmd.starts_with("vlan batch ") {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        let range_str = cmd.strip_prefix("vlan batch ").unwrap_or("").trim();
        return match parse_vlan_range(range_str) {
            Ok(vlans) => {
                let mut added = 0;
                for vlan in vlans {
                    if !device.vlans.contains(&vlan) {
                        device.vlans.push(vlan);
                        added += 1;
                    }
                }
                device.vlans.sort();
                Some(CommandResult {
                    success: true,
                    output: format!("{} VLANs created", added),
                    new_view: None,
                    new_hostname: None,
                })
            },
            Err(e) => Some(CommandResult {
                success: false,
                output: format!("Error: {}", e),
                new_view: None,
                new_hostname: None,
            }),
        };
    }
    
    // Delete VLAN
    if cmd.starts_with("undo vlan ") || cmd.starts_with("no vlan ") {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        
        let vlan_str = cmd.split_whitespace().last().unwrap_or("");
        return match vlan_str.parse::<u16>() {
            Ok(vlan) => {
                if vlan == 1 {
                    return Some(CommandResult {
                        success: false,
                        output: "Error: Cannot delete VLAN 1 (default)".to_string(),
                        new_view: None,
                        new_hostname: None,
                    });
                }
                if let Some(pos) = device.vlans.iter().position(|&v| v == vlan) {
                    device.vlans.remove(pos);
                    Some(CommandResult {
                        success: true,
                        output: format!("VLAN {} deleted", vlan),
                        new_view: None,
                        new_hostname: None,
                    })
                } else {
                    Some(CommandResult {
                        success: false,
                        output: format!("Error: VLAN {} does not exist", vlan),
                        new_view: None,
                        new_hostname: None,
                    })
                }
            },
            _ => Some(CommandResult {
                success: false,
                output: "Error: Invalid VLAN ID".to_string(),
                new_view: None,
                new_hostname: None,
            }),
        };
    }
    
    // VLAN name command (inside VLAN view ideally, but we allow in system-view)
    if cmd.starts_with("name ") && (*current_view == CliView::SystemView) {
        let name = cmd.strip_prefix("name ").unwrap_or("").trim();
        return Some(CommandResult {
            success: true,
            output: format!("VLAN name set to '{}'", name),
            new_view: None,
            new_hostname: None,
        });
    }
    
    None
}

/// Parse VLAN range like "10 to 20" or "10-20" or "10 20 30"
fn parse_vlan_range(input: &str) -> Result<Vec<u16>, String> {
    let mut vlans = Vec::new();
    
    // Handle "X to Y" format
    if input.contains(" to ") {
        let parts: Vec<&str> = input.split(" to ").collect();
        if parts.len() == 2 {
            let start: u16 = parts[0].trim().parse().map_err(|_| "Invalid start VLAN")?;
            let end: u16 = parts[1].trim().parse().map_err(|_| "Invalid end VLAN")?;
            if start > end || start < 1 || end > 4094 {
                return Err("Invalid VLAN range".to_string());
            }
            for v in start..=end {
                vlans.push(v);
            }
            return Ok(vlans);
        }
    }
    
    // Handle "X-Y" format
    if input.contains('-') && !input.contains(' ') {
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() == 2 {
            let start: u16 = parts[0].trim().parse().map_err(|_| "Invalid start VLAN")?;
            let end: u16 = parts[1].trim().parse().map_err(|_| "Invalid end VLAN")?;
            if start > end || start < 1 || end > 4094 {
                return Err("Invalid VLAN range".to_string());
            }
            for v in start..=end {
                vlans.push(v);
            }
            return Ok(vlans);
        }
    }
    
    // Handle space-separated list
    for part in input.split_whitespace() {
        let v: u16 = part.parse().map_err(|_| format!("Invalid VLAN: {}", part))?;
        if v < 1 || v > 4094 {
            return Err(format!("VLAN {} out of range (1-4094)", v));
        }
        vlans.push(v);
    }
    
    Ok(vlans)
}

/// Generate VLAN table display
fn generate_vlan_table(device: &NetworkDevice) -> String {
    let mut lines = Vec::new();
    lines.push(format!("{:<6} {:<20} {:<10}", "VLAN", "Name", "Status"));
    lines.push("-".repeat(40));
    
    // Always include VLAN 1
    lines.push(format!("{:<6} {:<20} {:<10}", "1", "default", "active"));
    
    for vlan in &device.vlans {
        if *vlan != 1 {
            lines.push(format!("{:<6} {:<20} {:<10}", vlan, format!("VLAN{:04}", vlan), "active"));
        }
    }
    
    lines.push("".to_string());
    lines.push(format!("Total VLANs: {}", device.vlans.len().max(1)));
    
    lines.join("\n")
}
