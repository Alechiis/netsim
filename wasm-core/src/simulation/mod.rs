use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::types::packet::Packet;
use crate::state::network_state::STATE;

lazy_static! {
    pub static ref SIMULATION_ENGINE: Mutex<SimulationEngine> = Mutex::new(SimulationEngine::new());
}

/// Discrete event for the simulation engine
#[derive(Debug)]
pub enum Event {
    PacketArrival { 
        to_device_id: String, 
        ingress_port: String, 
        packet: Packet 
    },
    TimerExpiry { 
        device_id: String, 
        timer_id: String 
    },
}

/// Simulation event with priority (timestamp)
#[derive(Debug)]
struct ScheduledEvent {
    time: u64,
    event: Event,
}

// Implement Ord for priority queue (min-heap via reverse ordering)
impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time) // Reverse for min-heap
    }
}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for ScheduledEvent {}

pub struct SimulationEngine {
    global_time: u64,
    event_queue: BinaryHeap<ScheduledEvent>,
}

impl SimulationEngine {
    pub fn new() -> Self {
        SimulationEngine {
            global_time: 0,
            event_queue: BinaryHeap::new(),
        }
    }
    
    pub fn schedule_event(&mut self, delay: u64, event: Event) {
        self.event_queue.push(ScheduledEvent {
            time: self.global_time + delay,
            event,
        });
    }
    
    /// Process all events up to current time (immediate execution for now)
    /// In a real loop, this would run step-by-step.
    pub fn process_packet_immediate(&mut self, src_dev_id: &str, packet: Packet) -> Option<Packet> {
        // 1. Find egress interface based on routing/switching
        // For Milestone 5.1 (Direct Connectivity), we just check cables
        
        // We need to release the lock immediately if we don't need it, but here we need it for the whole search
        let state_guard = STATE.lock().unwrap();
        
        // Find source device
        let src_dev = match state_guard.devices.get(src_dev_id) {
            Some(d) => d,
            None => return None,
        };

        // Simple Layer 2/3 Check logic for Ping
        // This is a simplified "Routing" logic for the MVP
        for port in &src_dev.ports {
            // Check based on Port ID, as cables use Port IDs
            let port_id = &port.id;
            
            // Check if this port is connected via cable as SOURCE
            if let Some(cable) = state_guard.cables.iter().find(|c| c.source_device_id == src_dev_id && &c.source_port_id == port_id) {
                // Connected to target_device
                if let Some(dst_dev) = state_guard.devices.get(&cable.target_device_id) {
                     // Check destination ports for IP match
                    for dst_port in &dst_dev.ports {
                        if let Some(target_ip) = &dst_port.config.ip_address {
                            if *target_ip == packet.dst_ip {
                                // SUCCESS
                                let mut reply = packet.clone();
                                reply.src_ip = packet.dst_ip.clone();
                                reply.dst_ip = packet.src_ip.clone();
                                return Some(reply);
                            }
                        }
                    }
                }
            } 
            // Check if this port is connected via cable as TARGET
            else if let Some(cable) = state_guard.cables.iter().find(|c| c.target_device_id == src_dev_id && &c.target_port_id == port_id) {
                // Connected to source_device
                if let Some(dst_dev) = state_guard.devices.get(&cable.source_device_id) {
                     // Check destination ports for IP match
                    for dst_port in &dst_dev.ports {
                        if let Some(target_ip) = &dst_port.config.ip_address {
                            if *target_ip == packet.dst_ip {
                                // SUCCESS
                                let mut reply = packet.clone();
                                reply.src_ip = packet.dst_ip.clone();
                                reply.dst_ip = packet.src_ip.clone();
                                return Some(reply);
                            }
                        }
                    }
                }
            }
        }
        
        None
    }
}
