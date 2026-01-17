// CLI Slice - Network Simulator Command Line Interface
import { StateCreator } from 'zustand';
import type {
  NetworkDevice,
  PacketTrace
} from '../../types/NetworkTypes.js';
import {
  recomputeOspf,
  cleanDhcpLeases
} from './createTopologySlice.js';
import {
  CliVendorProfileId
} from '../../utils/cliProfiles.js';
import { getCliStrategy } from '../../plugins/cliParser.js';
import { wasmExecuteCommand } from '../../wasm/wasmBridge';


// Import refactored CLI modules
import {
  initializeCommandRegistry,
  formatRunningConfig,
  executeCliCommand
} from './cli/index.js';

// Initialize registry once
initializeCommandRegistry();

// Interface for CliSlice
export interface CliSlice {
  commandUsage: Record<string, number>;
  activeVendorProfileId: CliVendorProfileId | null;
  currentCliStrategyId: string;
  activeCliStrategyDescription: string;
  executeCommand: (command: string) => void;
  setActiveVendorProfile: (profileId: CliVendorProfileId | null) => void;
  updateCliStrategy: (profileId: CliVendorProfileId | null) => void;
  exportRunningConfig: () => string;
  clearConsoleLogs: (id: string) => void;
  sessionHistory: Record<string, string[]>;
  addToHistory: (id: string, command: string) => void;
}

const defaultCliStrategy = getCliStrategy(null);

export const createCliSlice: StateCreator<any, [], [], CliSlice> = (set, get) => {
  let packetTraceTimer: ReturnType<typeof setTimeout> | null = null;
  const highlightTraffic = (path: string[], trace: PacketTrace | null) => {
    set({ activeTrafficParams: path, packetTrace: trace });
    if (path.length) {
      setTimeout(() => set({ activeTrafficParams: [] }), 900);
    }
    if (packetTraceTimer) {
      clearTimeout(packetTraceTimer);
    }
    if (trace) {
      packetTraceTimer = setTimeout(() => set({ packetTrace: null }), 4000);
    }
  };

  const buildDeviceRunningConfigSection = (dev: NetworkDevice): string[] => {
    const header = [`# Device ${dev.hostname} (${dev.model})`, `vendor ${dev.vendor}`];
    return [...header, ...formatRunningConfig(dev)];
  };

  const buildRunningConfigText = (devices: NetworkDevice[]): string => {
    const sections: string[] = [];
    devices.forEach((dev, index) => {
      sections.push(...buildDeviceRunningConfigSection(dev));
      if (index < devices.length - 1) sections.push('');
    });
    const text = sections.join('\n').trim();
    return text ? `${text}\n` : '';
  };

  const syncCliStrategy = (profileId: CliVendorProfileId | null) => {
    const strategy = getCliStrategy(profileId);
    const description = strategy.describe(profileId);
    set({ currentCliStrategyId: strategy.id, activeCliStrategyDescription: description });
    return strategy;
  };

  return {
    commandUsage: {},
    sessionHistory: {},
    activeVendorProfileId: null,
    currentCliStrategyId: defaultCliStrategy.id,
    activeCliStrategyDescription: defaultCliStrategy.describe(null),

    addToHistory: (id, cmd) => {
      set((state: any) => {
        const prev = state.sessionHistory[id] || [];
        return {
          sessionHistory: {
            ...state.sessionHistory,
            [id]: [...prev, cmd].slice(-50) // Keep last 50
          }
        };
      });
    },

    setActiveVendorProfile: (profileId: CliVendorProfileId | null) => {
      set({ activeVendorProfileId: profileId });
      syncCliStrategy(profileId);
    },
    updateCliStrategy: syncCliStrategy,
    exportRunningConfig: () => {
      const { devices } = get();
      return buildRunningConfigText(devices);
    },
    executeCommand: (cmdInput: string) => {
      const { activeConsoleId, devices, cables } = get();

      if (!activeConsoleId) return;

      const currentDevice = (devices as NetworkDevice[]).find((d: NetworkDevice) => d.id === activeConsoleId);
      if (currentDevice) {
        // Helper for cloning
        const cloneDeviceHelper = (id: string) => {
          const dev = (get().devices as NetworkDevice[]).find((d: NetworkDevice) => d.id === id);
          return dev ? JSON.parse(JSON.stringify(dev)) : undefined;
        };

        // Simple highlight adapter
        const highlightTrafficHelper = (path: string[], trace: any) => {
          highlightTraffic(path, trace);
        };

        // ALL commands are now handled via Command Pattern
        const history = get().sessionHistory[activeConsoleId] || [];

        // Map TS view to Rust view format
        const tsToRustViewMap: Record<string, string> = {
          'user-view': 'userView',
          'system-view': 'systemView',
          'interface-view': 'interfaceView',
          'bgp-view': 'bgpView',
          'pool-view': 'poolView',
        };
        const currentTsView = currentDevice.cliState?.view || 'user-view';
        const currentRustView = tsToRustViewMap[currentTsView] || 'userView';

        // Try WASM execution first
        const wasmResult = wasmExecuteCommand(activeConsoleId, cmdInput.trim(), currentRustView);

        // Handle WASM result
        if (wasmResult && wasmResult.success) {
          get().addToHistory(activeConsoleId, cmdInput.trim());
          set((state: any) => {
            const devIndex = state.devices.findIndex((d: NetworkDevice) => d.id === activeConsoleId);
            if (devIndex === -1) return state;

            const newDev = { ...state.devices[devIndex] };
            newDev.consoleLogs = [...newDev.consoleLogs, wasmResult.output];

            // Handle view change
            if (wasmResult.newView) {
              // Map Rust view to TS view (roughly)
              const viewMap: Record<string, string> = {
                'userView': 'user-view',
                'systemView': 'system-view',
                'interfaceView': 'interface-view'
              };
              newDev.cliState = { ...newDev.cliState, view: (viewMap[wasmResult.newView] || newDev.cliState.view) as any };
            }

            // Handle hostname change - sync to UI
            if (wasmResult.newHostname) {
              newDev.hostname = wasmResult.newHostname;
              // Also update the label for display
              newDev.data = { ...newDev.data, label: wasmResult.newHostname };
            }

            const newDevices = [...state.devices];
            newDevices[devIndex] = newDev;

            return { ...state, devices: newDevices };
          });
          return;
        }


        executeCliCommand(
          cmdInput.trim(),
          currentDevice,
          devices,
          cables,
          cloneDeviceHelper,
          highlightTrafficHelper,
          history
        ).then(result => {
          // Add to history
          get().addToHistory(activeConsoleId, cmdInput.trim());

          set((state: any) => {
            const devIndex = state.devices.findIndex((d: NetworkDevice) => d.id === activeConsoleId);
            if (devIndex === -1) return state;

            let newDev: NetworkDevice;

            if (result.device) {
              newDev = { ...result.device };
              // Ensure we don't duplicate logs if the command already appended them
              if (!newDev.consoleLogs.includes(result.output[result.output.length - 1])) {
                newDev.consoleLogs = [...newDev.consoleLogs, ...result.output];
              }
            } else {
              newDev = { ...state.devices[devIndex] };
              newDev.consoleLogs = [...newDev.consoleLogs, ...result.output];
            }

            let newDevices = [...state.devices];
            newDevices[newDev.id === activeConsoleId ? devIndex : newDevices.findIndex(d => d.id === newDev.id)] = newDev;

            // Apply global network logic
            newDevices = cleanDhcpLeases(newDevices);
            const recomputed = recomputeOspf(newDevices, state.cables);

            return {
              ...state,
              devices: recomputed.devices
            };
          });
        });
      }
    },
    clearConsoleLogs: (id: string) => {
      set((state: any) => ({
        devices: state.devices.map((d: any) =>
          d.id === id ? { ...d, consoleLogs: [] } : d
        )
      }));
    }
  };
};
