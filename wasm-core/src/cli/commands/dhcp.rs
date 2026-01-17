use crate::types::cli::{CliView, CommandResult};
use crate::types::network::NetworkDevice;

/// DHCP Pool structure
#[derive(Clone, Debug)]
pub struct DhcpPool {
    pub name: String,
    pub network: Option<String>,
    pub mask: Option<u8>,
    pub gateway: Option<String>,
    pub dns: Vec<String>,
    pub lease_days: u32,
    pub excluded: Vec<String>,
}

/// Helper to create error for wrong view
fn view_error() -> CommandResult {
    CommandResult {
        success: false,
        output: "Error: Command requires system-view. Enter 'system-view' first.".to_string(),
        new_view: None,
        new_hostname: None,
    }
}

/// Handle DHCP configuration commands
pub fn handle_dhcp_commands(device: &mut NetworkDevice, cmd: &str, current_view: &CliView) -> Option<CommandResult> {
    
    // ========== Display commands - allowed from any view ==========
    
    if cmd == "display ip pool" || cmd == "show ip dhcp pool" {
        let output = generate_dhcp_pool_display(device);
        return Some(CommandResult {
            success: true,
            output,
            new_view: None,
            new_hostname: None,
        });
    }
    
    if cmd == "display ip pool interface" || cmd == "show ip dhcp binding" {
        let output = generate_dhcp_binding_display();
        return Some(CommandResult {
            success: true,
            output,
            new_view: None,
            new_hostname: None,
        });
    }
    
    if cmd == "display dhcp server statistics" || cmd == "show ip dhcp server statistics" {
        let output = generate_dhcp_statistics();
        return Some(CommandResult {
            success: true,
            output,
            new_view: None,
            new_hostname: None,
        });
    }
    
    if cmd == "display dhcp server conflict" || cmd == "show ip dhcp conflict" {
        return Some(CommandResult {
            success: true,
            output: "No DHCP address conflicts detected.".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }
    
    // ========== Configuration commands - require system-view ==========
    
    // Enable DHCP globally
    if cmd == "dhcp enable" || cmd == "service dhcp" {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        device.dhcp_enabled = Some(true);
        return Some(CommandResult {
            success: true,
            output: "DHCP service enabled".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }
    
    // Disable DHCP
    if cmd == "undo dhcp enable" || cmd == "no service dhcp" {
        if *current_view != CliView::SystemView {
            return Some(view_error());
        }
        device.dhcp_enabled = Some(false);
        return Some(CommandResult {
            success: true,
            output: "DHCP service disabled".to_string(),
            new_view: None,
            new_hostname: None,
        });
    }
    
    // Create/enter DHCP pool (Huawei: ip pool, Cisco: ip dhcp pool)
    if cmd.starts_with("ip pool ") || cmd.starts_with("ip dhcp pool ") {
        let pool_name = if cmd.starts_with("ip pool ") {
            cmd.strip_prefix("ip pool ").unwrap_or("").trim()
        } else {
            cmd.strip_prefix("ip dhcp pool ").unwrap_or("").trim()
        };
        
        if pool_name.is_empty() {
            return Some(CommandResult {
                success: false,
                output: "Error: Pool name required".to_string(),
                new_view: None, new_hostname: None, });
        }
        
        return Some(CommandResult {
            success: true,
            output: format!("DHCP pool '{}' created. Entering pool configuration.", pool_name),
            new_view: Some(CliView::PoolView), new_hostname: None, });
    }
    
    // Delete DHCP pool
    if cmd.starts_with("undo ip pool ") || cmd.starts_with("no ip dhcp pool ") {
        let pool_name = cmd.split_whitespace().last().unwrap_or("");
        return Some(CommandResult {
            success: true,
            output: format!("DHCP pool '{}' deleted", pool_name),
            new_view: None, new_hostname: None, });
    }
    
    // ========== Pool configuration commands (inside pool-view) ==========
    
    // Network configuration (Huawei: network <ip> mask <mask>)
    if cmd.starts_with("network ") && cmd.contains("mask") {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        // network <ip> mask <mask>
        if parts.len() >= 4 {
            let network = parts[1];
            let mask = parts[3];
            
            if !is_valid_ipv4(network) {
                return Some(CommandResult {
                    success: false,
                    output: format!("Error: Invalid network address '{}'", network),
                    new_view: None, new_hostname: None, });
            }
            
            return Some(CommandResult {
                success: true,
                output: format!("Pool network set to {} mask {}", network, mask),
                new_view: None, new_hostname: None, });
        }
    }
    
    // Network configuration (Cisco: network <ip> <mask> or network <ip> /cidr)
    if cmd.starts_with("network ") && !cmd.contains("mask") && !cmd.contains("area") {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() >= 3 {
            let network = parts[1];
            let mask = parts[2];
            
            if !is_valid_ipv4(network) {
                return Some(CommandResult {
                    success: false,
                    output: format!("Error: Invalid network address '{}'", network),
                    new_view: None, new_hostname: None, });
            }
            
            return Some(CommandResult {
                success: true,
                output: format!("Pool network set to {} {}", network, mask),
                new_view: None, new_hostname: None, });
        }
    }
    
    // Gateway (Huawei: gateway-list, Cisco: default-router)
    if cmd.starts_with("gateway-list ") || cmd.starts_with("default-router ") {
        let gateway = cmd.split_whitespace().last().unwrap_or("");
        
        if !is_valid_ipv4(gateway) {
            return Some(CommandResult {
                success: false,
                output: format!("Error: Invalid gateway address '{}'", gateway),
                new_view: None, new_hostname: None, });
        }
        
        return Some(CommandResult {
            success: true,
            output: format!("Default gateway set to {}", gateway),
            new_view: None, new_hostname: None, });
    }
    
    // DNS (Huawei: dns-list, Cisco: dns-server)
    if cmd.starts_with("dns-list ") || cmd.starts_with("dns-server ") {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let dns_servers: Vec<&str> = parts[1..].to_vec();
        
        for dns in &dns_servers {
            if !is_valid_ipv4(dns) {
                return Some(CommandResult {
                    success: false,
                    output: format!("Error: Invalid DNS server address '{}'", dns),
                    new_view: None, new_hostname: None, });
            }
        }
        
        return Some(CommandResult {
            success: true,
            output: format!("DNS servers set to: {}", dns_servers.join(", ")),
            new_view: None, new_hostname: None, });
    }
    
    // Lease time
    if cmd.starts_with("lease ") {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() >= 2 {
            let days = parts[1];
            return Some(CommandResult {
                success: true,
                output: format!("Lease time set to {} days", days),
                new_view: None, new_hostname: None, });
        }
    }
    
    // Excluded addresses (Huawei: excluded-ip-address, Cisco: ip dhcp excluded-address)
    if cmd.starts_with("excluded-ip-address ") || cmd.starts_with("ip dhcp excluded-address ") {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() >= 2 {
            let start_ip = parts[parts.len() - 2];
            let end_ip = if parts.len() >= 3 { parts[parts.len() - 1] } else { start_ip };
            
            return Some(CommandResult {
                success: true,
                output: format!("Excluded addresses: {} - {}", start_ip, end_ip),
                new_view: None, new_hostname: None, });
        }
    }
    
    // Domain name
    if cmd.starts_with("domain-name ") {
        let domain = cmd.strip_prefix("domain-name ").unwrap_or("").trim();
        return Some(CommandResult {
            success: true,
            output: format!("Domain name set to '{}'", domain),
            new_view: None, new_hostname: None, });
    }
    
    // ========== Display commands ==========
    
    // Display DHCP pools
    if cmd == "display ip pool" || cmd == "show ip dhcp pool" {
        let output = generate_dhcp_pool_display(device);
        return Some(CommandResult {
            success: true,
            output,
            new_view: None, new_hostname: None, });
    }
    
    // Display DHCP bindings/leases
    if cmd == "display ip pool interface" || cmd == "show ip dhcp binding" {
        let output = generate_dhcp_binding_display();
        return Some(CommandResult {
            success: true,
            output,
            new_view: None, new_hostname: None, });
    }
    
    // Display DHCP server statistics
    if cmd == "display dhcp server statistics" || cmd == "show ip dhcp server statistics" {
        let output = generate_dhcp_statistics();
        return Some(CommandResult {
            success: true,
            output,
            new_view: None, new_hostname: None, });
    }
    
    // Display DHCP conflicts
    if cmd == "display dhcp server conflict" || cmd == "show ip dhcp conflict" {
        return Some(CommandResult {
            success: true,
            output: "No DHCP address conflicts detected.".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // Clear DHCP bindings
    if cmd == "reset ip pool" || cmd == "clear ip dhcp binding *" {
        return Some(CommandResult {
            success: true,
            output: "All DHCP bindings cleared.".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // DHCP relay configuration
    if cmd.starts_with("dhcp relay server-ip ") || cmd.starts_with("ip helper-address ") {
        let server_ip = cmd.split_whitespace().last().unwrap_or("");
        
        if !is_valid_ipv4(server_ip) {
            return Some(CommandResult {
                success: false,
                output: format!("Error: Invalid server address '{}'", server_ip),
                new_view: None, new_hostname: None, });
        }
        
        return Some(CommandResult {
            success: true,
            output: format!("DHCP relay configured to forward to {}", server_ip),
            new_view: None, new_hostname: None, });
    }
    
    // Enable DHCP relay on interface
    if cmd == "dhcp select relay" {
        return Some(CommandResult {
            success: true,
            output: "DHCP relay enabled on interface".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    // Enable DHCP server on interface
    if cmd == "dhcp select global" || cmd == "dhcp select interface" {
        return Some(CommandResult {
            success: true,
            output: "DHCP server enabled on interface".to_string(),
            new_view: None, new_hostname: None, });
    }
    
    None
}

/// Generate DHCP pool display
fn generate_dhcp_pool_display(device: &NetworkDevice) -> String {
    let mut lines = Vec::new();
    
    let enabled = device.dhcp_enabled.unwrap_or(false);
    
    lines.push("DHCP Server Pool Information".to_string());
    lines.push(format!("DHCP Service: {}", if enabled { "Enabled" } else { "Disabled" }));
    lines.push("".to_string());
    
    if !enabled {
        lines.push("(DHCP service is not enabled. Use 'dhcp enable' to start.)".to_string());
        return lines.join("\n");
    }
    
    lines.push(format!("{:<15} {:<18} {:<15} {:<10}", "Pool Name", "Network", "Gateway", "Leases"));
    lines.push("-".repeat(60));
    
    // Simulated pool data
    lines.push(format!("{:<15} {:<18} {:<15} {:<10}", "LAN_POOL", "192.168.1.0/24", "192.168.1.1", "0/254"));
    lines.push(format!("{:<15} {:<18} {:<15} {:<10}", "GUEST_POOL", "10.0.0.0/24", "10.0.0.1", "0/254"));
    
    lines.push("".to_string());
    lines.push("(Showing simulated pools - configure with 'ip pool <name>')".to_string());
    
    lines.join("\n")
}

/// Generate DHCP binding display
fn generate_dhcp_binding_display() -> String {
    let mut lines = Vec::new();
    
    lines.push("DHCP Address Bindings".to_string());
    lines.push("".to_string());
    lines.push(format!("{:<16} {:<18} {:<12} {:<20}", "IP Address", "MAC Address", "Type", "Lease Expires"));
    lines.push("-".repeat(70));
    
    // Simulated bindings
    lines.push(format!("{:<16} {:<18} {:<12} {:<20}", 
        "192.168.1.100", "00:50:56:C0:00:01", "Dynamic", "Jan 18 2026 12:00"));
    lines.push(format!("{:<16} {:<18} {:<12} {:<20}", 
        "192.168.1.101", "00:50:56:C0:00:02", "Dynamic", "Jan 18 2026 14:30"));
    lines.push(format!("{:<16} {:<18} {:<12} {:<20}", 
        "192.168.1.50", "00:50:56:C0:00:10", "Static", "Infinite"));
    
    lines.push("".to_string());
    lines.push("Total bindings: 3".to_string());
    
    lines.join("\n")
}

/// Generate DHCP statistics
fn generate_dhcp_statistics() -> String {
    format!(
        "DHCP Server Statistics\n\
         \n\
         Message         Received    Sent\n\
         ---------------------------------\n\
         DISCOVER        15          0\n\
         OFFER           0           15\n\
         REQUEST         12          0\n\
         ACK             0           12\n\
         NAK             0           0\n\
         DECLINE         0           0\n\
         RELEASE         3           0\n\
         INFORM          0           0\n\
         \n\
         Total leases: 3\n\
         Available addresses: 251\n\
         Utilization: 1.2%"
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

