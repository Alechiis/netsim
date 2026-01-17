pub mod types;
pub mod state;
pub mod cli;
pub mod protocols;
pub mod simulation;

use wasm_bindgen::prelude::*;
use crate::state::network_state::STATE;
use crate::types::network::{NetworkDevice, NetworkCable};
use crate::types::cli::CliView;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn set_topology(devices_val: JsValue, cables_val: JsValue) -> Result<(), JsValue> {
    let devices: Vec<NetworkDevice> = serde_wasm_bindgen::from_value(devices_val)?;
    let cables: Vec<NetworkCable> = serde_wasm_bindgen::from_value(cables_val)?;
    
    let mut state = STATE.lock().expect("Failed to lock STATE");
    state.devices.clear();
    for device in devices {
        state.devices.insert(device.id.clone(), device);
    }
    state.cables = cables;
    
    Ok(())
}

#[wasm_bindgen]
pub fn get_devices() -> Result<JsValue, JsValue> {
    let state = STATE.lock().expect("Failed to lock STATE");
    let devices: Vec<&NetworkDevice> = state.devices.values().collect();
    Ok(serde_wasm_bindgen::to_value(&devices)?)
}

#[wasm_bindgen]
pub fn execute_command(device_id: &str, command: &str, current_view: JsValue) -> Result<JsValue, JsValue> {
    // Parse the current CLI view from TypeScript
    let view: CliView = serde_wasm_bindgen::from_value(current_view).unwrap_or(CliView::UserView);
    
    let result = crate::cli::executor::execute_command(device_id, command, view);
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

#[wasm_bindgen]
pub fn ping(input: &str) -> String {
    format!("WASM Engine Received: {}", input)
}

