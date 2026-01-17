// @ts-ignore - WASM module doesn't have TypeScript declarations
import init, { ping, init_panic_hook, set_topology, get_devices, execute_command } from './pkg/wasm_core.js';

let wasmReady = false;

export const initWasm = async () => {
    if (wasmReady) return;
    try {
        await init();
        init_panic_hook();
        wasmReady = true;
        console.log('🚀 WASM Engine Initialized');
    } catch (error) {
        console.error('❌ Failed to initialize WASM Engine:', error);
        throw error;
    }
};

export const wasmSetTopology = (devices: any[], cables: any[]) => {
    if (!wasmReady) throw new Error('WASM not ready');
    return set_topology(devices, cables);
};

export const wasmGetDevices = () => {
    if (!wasmReady) throw new Error('WASM not ready');
    return get_devices();
};

export const wasmExecuteCommand = (deviceId: string, command: string, currentView: string = 'userView') => {
    if (!wasmReady) throw new Error('WASM not ready');
    return execute_command(deviceId, command, currentView);
};

export const wasmPing = (input: string): string => {
    if (!wasmReady) throw new Error('WASM not ready');
    return ping(input);
};
