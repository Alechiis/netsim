use crate::types::cli::{CliView, CommandResult};
use crate::types::network::NetworkDevice;

/// Handle STP (Spanning Tree Protocol) configuration commands
pub fn handle_stp_commands(_device: &mut NetworkDevice, cmd: &str, _current_view: &CliView) -> Option<CommandResult> {
    
    // Enable STP globally
    if cmd == "stp enable" || cmd == "spanning-tree" {
        return Some(CommandResult {
            success: true,
            output: "Spanning Tree Protocol enabled".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // Disable STP
    if cmd == "undo stp enable" || cmd == "no spanning-tree" {
        return Some(CommandResult {
            success: true,
            output: "Spanning Tree Protocol disabled".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // Set STP mode (Huawei)
    if cmd.starts_with("stp mode ") {
        let mode = cmd.strip_prefix("stp mode ").unwrap_or("rstp").trim();
        let valid_modes = ["stp", "rstp", "mstp"];
        if valid_modes.contains(&mode) {
            return Some(CommandResult {
                success: true,
                output: format!("STP mode set to {}", mode.to_uppercase()),
                new_view: None, new_hostname: None, });
        } else {
            return Some(CommandResult {
                success: false,
                output: format!("Error: Invalid mode '{}'. Use stp, rstp, or mstp.", mode),
                new_view: None, new_hostname: None, });
        }
    }
    
    // Set STP mode (Cisco)
    if cmd.starts_with("spanning-tree mode ") {
        let mode = cmd.strip_prefix("spanning-tree mode ").unwrap_or("").trim();
        return Some(CommandResult {
            success: true,
            output: format!("Spanning tree mode set to {}", mode),
            new_view: None, new_hostname: None, });
    }
    
    // Set bridge priority (Huawei)
    if cmd.starts_with("stp priority ") {
        let priority = cmd.strip_prefix("stp priority ").unwrap_or("32768").trim();
        match priority.parse::<u32>() {
            Ok(p) if p <= 61440 && p % 4096 == 0 => {
                return Some(CommandResult {
                    success: true,
                    output: format!("Bridge priority set to {}", p),
                    new_view: None, new_hostname: None, });
            }
            _ => {
                return Some(CommandResult {
                    success: false,
                    output: "Error: Priority must be 0-61440 in increments of 4096".to_string(),
                    new_view: None, new_hostname: None, });
            }
        }
    }
    
    // Set bridge priority (Cisco)
    if cmd.starts_with("spanning-tree vlan ") && cmd.contains("priority") {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if let Some(priority_idx) = parts.iter().position(|&p| p == "priority") {
            if parts.len() > priority_idx + 1 {
                let priority = parts[priority_idx + 1];
                return Some(CommandResult {
                    success: true,
                    output: format!("VLAN bridge priority set to {}", priority),
                    new_view: None, new_hostname: None, });
            }
        }
    }
    
    // Force root bridge (Huawei)
    if cmd == "stp root primary" {
        return Some(CommandResult {
            success: true,
            output: "This switch is configured as root bridge (priority 0)".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // Force root bridge (Cisco)
    if cmd.starts_with("spanning-tree vlan") && cmd.contains("root primary") {
        return Some(CommandResult {
            success: true,
            output: "This switch is configured as root bridge for specified VLANs".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // Secondary root
    if cmd == "stp root secondary" || cmd.contains("root secondary") {
        return Some(CommandResult {
            success: true,
            output: "This switch is configured as secondary root bridge".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // Port cost
    if cmd.starts_with("stp cost ") || cmd.starts_with("spanning-tree cost ") {
        let cost = cmd.split_whitespace().last().unwrap_or("1");
        return Some(CommandResult {
            success: true,
            output: format!("Port cost set to {}", cost),
            new_view: None, new_hostname: None, });
    }
    
    // Port priority
    if cmd.starts_with("stp port-priority ") || cmd.starts_with("spanning-tree port-priority ") {
        let priority = cmd.split_whitespace().last().unwrap_or("128");
        return Some(CommandResult {
            success: true,
            output: format!("Port priority set to {}", priority),
            new_view: None, new_hostname: None, });
    }
    
    // Edge port (Huawei)
    if cmd == "stp edged-port enable" {
        return Some(CommandResult {
            success: true,
            output: "Port configured as edge port (fast transition to forwarding)".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // Portfast (Cisco)
    if cmd == "spanning-tree portfast" {
        return Some(CommandResult {
            success: true,
            output: "PortFast enabled on interface".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // BPDU protection (Huawei)
    if cmd == "stp bpdu-protection" {
        return Some(CommandResult {
            success: true,
            output: "BPDU protection enabled".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // BPDU guard (Cisco)
    if cmd == "spanning-tree bpduguard enable" || cmd == "spanning-tree portfast bpduguard default" {
        return Some(CommandResult {
            success: true,
            output: "BPDU guard enabled".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // Root guard
    if cmd == "stp root-protection" || cmd == "spanning-tree guard root" {
        return Some(CommandResult {
            success: true,
            output: "Root guard enabled on port".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // Loop guard
    if cmd == "stp loop-protection" || cmd == "spanning-tree guard loop" {
        return Some(CommandResult {
            success: true,
            output: "Loop guard enabled on port".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // Display STP
    if cmd == "display stp" || cmd == "show spanning-tree" {
        let output = generate_stp_display();
        return Some(CommandResult {
            success: true,
            output,
            new_view: None, new_hostname: None, });
    }
    
    // Display STP brief
    if cmd == "display stp brief" || cmd == "show spanning-tree summary" {
        let output = generate_stp_summary();
        return Some(CommandResult {
            success: true,
            output,
            new_view: None, new_hostname: None, });
    }
    
    // Display STP interface
    if cmd.starts_with("display stp interface") || cmd.starts_with("show spanning-tree interface") {
        let iface = cmd.split_whitespace().last().unwrap_or("");
        return Some(CommandResult {
            success: true,
            output: format!(
                "STP Port State for {}\n\
                 Port Role: Designated\n\
                 Port State: Forwarding\n\
                 Port Cost: 20000\n\
                 Port Priority: 128\n\
                 Designated Bridge: 32768.0050.5600.0001",
                iface
            ),
            new_view: None, new_hostname: None, });
    }
    
    None
}

/// Generate STP display
fn generate_stp_display() -> String {
    format!(
        "Spanning Tree Protocol Status\n\
         \n\
         Mode: RSTP (Rapid Spanning Tree)\n\
         Bridge ID: 32768.0050.5600.0001\n\
         Root Bridge: 32768.0050.5600.0001 (This bridge is root)\n\
         \n\
         Interface       Role       State      Cost   Priority\n\
         -------------------------------------------------------\n\
         GE0/0/1         Designated Forwarding 20000  128\n\
         GE0/0/2         Designated Forwarding 20000  128\n\
         GE0/0/3         Designated Forwarding 20000  128\n\
         GE0/0/4         Root       Forwarding 20000  128\n\
         \n\
         Forward Delay: 15s, Max Age: 20s, Hello Time: 2s"
    )
}

/// Generate STP summary
fn generate_stp_summary() -> String {
    format!(
        "STP Summary\n\
         \n\
         Bridge Mode: RSTP\n\
         Root Bridge: Yes (this switch)\n\
         Bridge Priority: 32768\n\
         Bridge MAC: 0050.5600.0001\n\
         Total Ports: 24\n\
         Forwarding: 4\n\
         Blocking: 0\n\
         Topology Changes: 0"
    )
}

