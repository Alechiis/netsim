use crate::types::cli::{CliView, CommandResult};
use crate::types::network::NetworkDevice;

pub fn handle_system_commands(device: &mut NetworkDevice, cmd: &str, current_view: &CliView) -> Option<CommandResult> {
    // Navigation commands - always available
    if cmd == "return" || cmd == "end" {
        return Some(CommandResult {
            success: true,
            output: "".to_string(),
            new_view: Some(CliView::UserView),
            new_hostname: None,
        });
    }

    // Sysname / Hostname commands - REQUIRE SystemView
    if cmd.starts_with("sysname ") || cmd.starts_with("hostname ") {
        // Check if we are in system-view
        if *current_view != CliView::SystemView {
            return Some(CommandResult {
                success: false,
                output: "Error: Command requires system-view. Enter 'system-view' first.".to_string(),
                new_view: None,
                new_hostname: None,
            });
        }
        
        let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
        if parts.len() < 2 || parts[1].trim().is_empty() {
            return Some(CommandResult {
                success: false,
                output: "Error: Hostname cannot be empty.".to_string(),
                new_view: None,
                new_hostname: None,
            });
        }
        let new_name = parts[1].trim().to_string();
        // Validate hostname (alphanumeric, hyphens, max 64 chars)
        if new_name.len() > 64 {
            return Some(CommandResult {
                success: false,
                output: "Error: Hostname too long (max 64 characters).".to_string(),
                new_view: None,
                new_hostname: None,
            });
        }
        if !new_name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Some(CommandResult {
                success: false,
                output: "Error: Invalid hostname. Use alphanumeric characters, hyphens, or underscores only.".to_string(),
                new_view: None,
                new_hostname: None,
            });
        }
        device.hostname = new_name.clone();
        return Some(CommandResult {
            success: true,
            output: format!("Hostname set to '{}'", new_name),
            new_view: None,
            new_hostname: Some(new_name), // Return new hostname for UI sync
        });
    }

    // Undo sysname / no hostname - REQUIRE SystemView
    if cmd == "undo sysname" || cmd == "no hostname" {
        if *current_view != CliView::SystemView {
            return Some(CommandResult {
                success: false,
                output: "Error: Command requires system-view. Enter 'system-view' first.".to_string(),
                new_view: None,
                new_hostname: None,
            });
        }
        let default_name = "Router".to_string();
        device.hostname = default_name.clone();
        return Some(CommandResult {
            success: true,
            output: "Hostname reset to default.".to_string(),
            new_view: None,
            new_hostname: Some(default_name),
        });
    }

    match cmd {
        "system-view" | "configure terminal" | "conf t" => Some(CommandResult {
            success: true,
            output: "Enter configuration commands, one per line. End with CNTL/Z.".to_string(),
            new_view: Some(CliView::SystemView),
            new_hostname: None,
        }),

        "display version" | "show version" => Some(CommandResult {
            success: true,
            output: format!(
                "NetSim OS Software, Version 1.0.0\n\
                 Copyright (C) 2024-2026 NetSim Community\n\
                 \n\
                 Device:    {}\n\
                 Model:     {}\n\
                 Vendor:    {:?}\n\
                 Uptime:    0 days, 0 hours, 0 minutes\n\
                 WASM Engine: Rust/WebAssembly v0.1.0",
                device.hostname, device.model, device.vendor
            ),
            new_view: None,
            new_hostname: None,
        }),

        "display current-configuration" | "show running-config" | "show run" => {
            let config = generate_running_config(device);
            Some(CommandResult {
                success: true,
                output: config,
                new_view: None,
                new_hostname: None,
            })
        },

        "display saved-configuration" | "show startup-config" => Some(CommandResult {
            success: true,
            output: format!(
                "!\n! Last saved configuration\n!\nhostname {}\n!\n! (Saved configuration not available in simulation)",
                device.hostname
            ),
            new_view: None,
            new_hostname: None,
        }),

        "save" | "write" | "write memory" | "copy running-config startup-config" => Some(CommandResult {
            success: true,
            output: "Configuration saved successfully.".to_string(),
            new_view: None,
            new_hostname: None,
        }),

        "display history-command" | "show history" => Some(CommandResult {
            success: true,
            output: "(Command history feature not yet implemented in WASM engine)".to_string(),
            new_view: None,
            new_hostname: None,
        }),

        "?" | "help" => Some(CommandResult {
            success: true,
            output: format!(
                "Available commands:\n\
                 \n\
                 Navigation:\n\
                 - system-view / configure terminal  Enter configuration mode\n\
                 - return / end                      Return to user view\n\
                 - exit / quit                       Exit current view\n\
                 \n\
                 System:\n\
                 - sysname <name> / hostname <name>  Set device hostname (requires system-view)\n\
                 - display version / show version   Display system version\n\
                 - display current-configuration    Show running config\n\
                 - save / write memory              Save configuration\n\
                 \n\
                 Interface:\n\
                 - interface <name>                 Enter interface config\n\
                 - display ip interface brief       Show IP summary\n\
                 "
            ),
            new_view: None,
            new_hostname: None,
        }),

        _ => None,
    }
}

/// Generate a running-config style output
fn generate_running_config(device: &NetworkDevice) -> String {
    let mut lines = Vec::new();
    
    lines.push("!".to_string());
    lines.push(format!("! NetSim Configuration - {}", device.hostname));
    lines.push("! Generated by WASM Engine".to_string());
    lines.push("!".to_string());
    lines.push(format!("hostname {}", device.hostname));
    lines.push("!".to_string());
    
    // VLANs
    if !device.vlans.is_empty() {
        for vlan in &device.vlans {
            lines.push(format!("vlan {}", vlan));
        }
        lines.push("!".to_string());
    }
    
    // Interfaces
    for port in &device.ports {
        lines.push(format!("interface {}", port.name));
        
        if let Some(desc) = &port.config.description {
            lines.push(format!(" description {}", desc));
        }
        
        match port.config.mode {
            crate::types::network::PortMode::Access => {
                lines.push(" port link-type access".to_string());
                if let Some(vlan) = port.config.vlan {
                    lines.push(format!(" port default vlan {}", vlan));
                }
            },
            crate::types::network::PortMode::Trunk => {
                lines.push(" port link-type trunk".to_string());
                if let Some(vlans) = &port.config.allowed_vlans {
                    let vlan_str: Vec<String> = vlans.iter().map(|v| v.to_string()).collect();
                    lines.push(format!(" port trunk allow-pass vlan {}", vlan_str.join(" ")));
                }
            },
            crate::types::network::PortMode::Hybrid => {
                lines.push(" port link-type hybrid".to_string());
            },
            crate::types::network::PortMode::Routed => {
                if let Some(ip) = &port.config.ip_address {
                    let mask = port.config.subnet_mask.unwrap_or(24);
                    lines.push(format!(" ip address {} {}", ip, cidr_to_mask(mask)));
                }
            },
        }
        
        if !port.config.enabled {
            lines.push(" shutdown".to_string());
        }
        
        lines.push("!".to_string());
    }
    
    // OSPF
    if device.ospf_enabled.unwrap_or(false) {
        lines.push("router ospf 1".to_string());
        lines.push(" area 0".to_string());
        lines.push("!".to_string());
    }
    
    // BGP
    if device.bgp_enabled.unwrap_or(false) {
        lines.push("bgp".to_string());
        lines.push("!".to_string());
    }
    
    lines.push("!".to_string());
    lines.push("end".to_string());
    
    lines.join("\n")
}

/// Convert CIDR prefix to dotted decimal mask
fn cidr_to_mask(cidr: u8) -> String {
    if cidr > 32 {
        return "255.255.255.255".to_string();
    }
    let mask: u32 = if cidr == 0 { 0 } else { !0u32 << (32 - cidr) };
    format!(
        "{}.{}.{}.{}",
        (mask >> 24) & 0xFF,
        (mask >> 16) & 0xFF,
        (mask >> 8) & 0xFF,
        mask & 0xFF
    )
}

