use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
// TypeScript sends "Switch", "Router", "PC" (PascalCase).
// We remove `rename_all = "camelCase"` for this enum or use equivalent manual renames if needed.
// Actually, `camelCase` converts `Switch` -> `switch`.
// The error says "expected switch...".
// It seems Serde with `rename_all="camelCase"` expects inputs to be camelCased.
// But our TS types are PascalCase: 'Switch', 'Router'.
// So we should just remove the attribute or use `PascalCase` if that matches.
// Given TS sends 'Switch', 'Router', let's just match that.
pub enum DeviceType {
    Switch,
    Router,
    PC,
    AP,
    Firewall,
    Wireless,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// TS: 'Huawei' | 'Cisco' | 'D-Link' | 'NetSim' | 'PC' | 'Router' | 'Aruba' | 'MikroTik'
pub enum Vendor {
    Huawei,
    Cisco,
    #[serde(rename = "D-Link")]
    DLink,
    NetSim,
    PC,
    Router,
    Aruba,
    MikroTik,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// TS: 'RJ45' | 'SFP' | 'Console'
pub enum PortType {
    RJ45,
    SFP,
    Console,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")] // TS: 'copper' | 'fiber'
pub enum CableType {
    Copper,
    Fiber,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")] // TS: 'up' | 'down'
pub enum LinkStatus {
    Up,
    Down,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq)]
#[serde(rename_all = "lowercase")] // TS: 'access' | 'trunk' | 'hybrid' | 'routed'
pub enum PortMode {
    Access,
    Trunk,
    Hybrid,
    Routed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PortConfig {
    pub vlan: Option<u16>,
    pub allowed_vlans: Option<Vec<u16>>,
    pub mode: PortMode,
    pub ip_address: Option<String>,
    pub subnet_mask: Option<u8>,
    pub enabled: bool,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPort {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub port_type: PortType,
    pub status: LinkStatus,
    pub config: PortConfig,
    pub connected_cable_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkDevice {
    pub id: String,
    #[serde(rename = "type")]
    pub device_type: DeviceType,
    pub vendor: Vendor,
    pub hostname: String,
    pub model: String,
    pub ports: Vec<NetworkPort>,
    pub vlans: Vec<u16>,
    pub ospf_enabled: Option<bool>,
    pub bgp_enabled: Option<bool>,
    pub dhcp_enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkCable {
    pub id: String,
    #[serde(rename = "type")]
    pub cable_type: CableType,
    pub source_device_id: String,
    pub source_port_id: String,
    pub target_device_id: String,
    pub target_port_id: String,
}
