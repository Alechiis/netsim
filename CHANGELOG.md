# Changelog

All notable changes to NetSim will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-01-17

### 🚀 Major Features

#### WebAssembly (WASM) CLI Engine
- **New Rust/WASM Engine**: Complete rewrite of CLI command execution in Rust, compiled to WebAssembly for near-native performance
- **10 Command Modules**: System, Interface, VLAN, Routing (OSPF, BGP, Static), PC/Host, DHCP, Security/ACL, STP, and LAG
- **~2,500 lines of Rust**: Robust command parsing and execution with proper error handling

#### CLI View Validation
- **Realistic CLI Hierarchy**: Commands now require the correct CLI view to execute
  - `sysname`/`hostname` → requires `system-view`
  - `interface GE0/0/1` → requires `system-view`
  - `vlan 10` → requires `system-view`
  - `ip address` → requires `interface-view`
  - `ip route-static` → requires `system-view`
  - `ospf`/`bgp` → requires `system-view`
- **Proper Exit Navigation**: `exit`/`quit` now correctly navigates back one level (interface → system → user)

#### Hostname Synchronization
- **UI Sync**: When hostname is changed via CLI, the device label updates immediately in the canvas and Node Inspector
- **New `newHostname` Field**: Added to `CommandResult` for WASM→TypeScript state synchronization

### 🔧 Technical Changes

- Added `wasm-core/` Rust crate with full CLI implementation
- Updated `wasmBridge.ts` to pass current CLI view to WASM
- Updated `createCliSlice.ts` to handle view mapping and hostname sync
- Build script now includes `npm run build:wasm` for WASM compilation

### 📦 Dependencies

- Added `wasm-pack` for WASM compilation (dev)

---

## [1.0.0] - 2026-01-16

### 🎉 Initial Release

- First public release of NetSim Community Edition
- Complete network simulation with Huawei VRP and Cisco IOS syntax support
- React/TypeScript frontend with ReactFlow canvas
- Device Manager with Routers, Switches, and PCs
- CLI Terminal with command history and autocomplete
- VLAN, Interface, and Routing configuration
- Labs system with interactive exercises
- AI-powered Assistant Coach
- 3D Network Visualization
- NETCONF and RESTCONF simulation
- Collaborative editing support
- Knowledge Base with RAG AI search
