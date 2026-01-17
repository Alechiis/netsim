# Release Notes - NetSim v1.1.0 (WASM Engine)

## Overview
This release marks a significant milestone in the NetSim project with the introduction of the WebAssembly (WASM) CLI Engine. The core command execution logic has been migrated from TypeScript to Rust, offering improved performance and a more robust foundation for future protocol implementations.

## Key Features

### 🚀 WebAssembly (WASM) CLI Engine
- **Core Migration**: CLI command parsing and execution logic is now handled by a new Rust-based engine (`wasm-core` crate).
- **View Validation**: Implemented strict view hierarchies (User, System, Interface, VLAN, etc.) to mimic real-world device behavior. Commands are now validated against the current view.
- **Enhanced Command Support**:
  - **System**: `sysname`, `hostname`
  - **Interfaces**: `interface`, `ip address`, `shutdown`, `description`, `port link-type`
  - **VLANs**: `vlan`, `vlan batch`, `display vlan`
  - **Routing**: `ip route-static`, `ospf`, `bgp`
  - **DHCP**: Pool creation and configuration
  - **Security**: ACLs and traffic filters
- **Performance**: Near-native execution speed for command processing.

### 🔄 State Synchronization
- **Hostname Sync**: Seamless synchronization of device hostnames between the WASM core and the React UI Canvas.
- **View Awareness**: The UI now reflects the current CLI view state managed by the WASM engine.

### 🛠 Technical Improvements
- **Rust Integration**: Project structure now includes a `wasm-core` Rust workspace.
- **Build Pipeline**: Updated `package.json` with `build:wasm` scripts.
- **Type Safety**: Improved type alignment between TypeScript and Rust models (e.g., `CableType` standardization).

## Breaking Changes
- `CableType` values `Copper` and `Fiber` have been standardized to lowercase `copper` and `fiber` across the codebase to ensure type safety between Rust and TypeScript.

## Known Issues
- Protocol simulation (OSPF/BGP routing logic propagation) is mocked in this release; full protocol engines are scheduled for v1.2.0.

## Usage
To build the project with the new WASM engine:
```bash
npm run build
```
This command will compile the Rust code to WASM and then build the frontend application.
