use crate::types::cli::{CliView, CommandResult};
use crate::state::network_state::STATE;
use crate::cli::commands::system::handle_system_commands;
use crate::cli::commands::interface::handle_interface_commands;
use crate::cli::commands::vlan::handle_vlan_commands;
use crate::cli::commands::routing::handle_routing_commands;
use crate::cli::commands::pc::handle_pc_commands;
use crate::cli::commands::dhcp::handle_dhcp_commands;
use crate::cli::commands::security::handle_security_commands;
use crate::cli::commands::stp::handle_stp_commands;
use crate::cli::commands::lag::handle_lag_commands;

pub fn execute_command(device_id: &str, command: &str, current_view: CliView) -> CommandResult {
    let mut state = match STATE.lock() {
        Ok(s) => s,
        Err(e) => return CommandResult {
            success: false,
            output: format!("Internal Error: {}", e),
            new_view: None,
            new_hostname: None,
        },
    };

    let device = match state.devices.get_mut(device_id) {
        Some(d) => d,
        None => return CommandResult {
            success: false,
            output: "Device not found".to_string(),
            new_view: None,
            new_hostname: None,
        },
    };

    let cmd = command.trim().to_lowercase();

    // Check generic navigation - always allowed
    if cmd == "exit" || cmd == "quit" {
        // Go back one level based on current view
        let new_view = match current_view {
            CliView::InterfaceView => Some(CliView::SystemView),
            CliView::SystemView => Some(CliView::UserView),
            CliView::BgpView => Some(CliView::SystemView),
            CliView::PoolView => Some(CliView::SystemView),
            CliView::AclView => Some(CliView::SystemView),
            _ => Some(CliView::UserView),
        };
        return CommandResult {
            success: true,
            output: "".to_string(),
            new_view,
            new_hostname: None,
        };
    }

    // Try system commands (pass current view for validation)
    if let Some(res) = handle_system_commands(device, &cmd, &current_view) {
        return res;
    }
    
    // Try interface commands (pass current view for validation)
    if let Some(res) = handle_interface_commands(device, &cmd, &current_view) {
        return res;
    }

    // Try VLAN commands (pass current view for validation)
    if let Some(res) = handle_vlan_commands(device, &cmd, &current_view) {
        return res;
    }

    // Try routing commands (pass current view for validation)
    if let Some(res) = handle_routing_commands(device, &cmd, &current_view) {
        return res;
    }

    // Try PC/Host commands (no view validation needed - these are user-level)
    if let Some(res) = handle_pc_commands(device, &cmd) {
        return res;
    }

    // Try DHCP commands (pass current view for validation)
    if let Some(res) = handle_dhcp_commands(device, &cmd, &current_view) {
        return res;
    }

    // Try Security/ACL commands (pass current view for validation)
    if let Some(res) = handle_security_commands(device, &cmd, &current_view) {
        return res;
    }

    // Try STP commands (pass current view for validation)
    if let Some(res) = handle_stp_commands(device, &cmd, &current_view) {
        return res;
    }

    // Try LAG/Link Aggregation commands (pass current view for validation)
    if let Some(res) = handle_lag_commands(device, &cmd, &current_view) {
        return res;
    }

    // Fallback info for non-migrated commands
    CommandResult {
        success: false,
        output: format!("Error: Unrecognized command '{}' (Engine Fallback Triggered)", command),
        new_view: None,
        new_hostname: None,
    }
}

