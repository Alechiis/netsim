use crate::types::network::{NetworkDevice, NetworkCable};
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub struct NetworkState {
    pub devices: HashMap<String, NetworkDevice>,
    pub cables: Vec<NetworkCable>,
}

impl NetworkState {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            cables: Vec::new(),
        }
    }
}

pub static STATE: Lazy<Mutex<NetworkState>> = Lazy::new(|| {
    Mutex::new(NetworkState::new())
});
