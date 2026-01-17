# NetSim Community Edition

<div align="center">
  <img src="public/logo.png" alt="NetSim Logo" width="120" />
  <h1>The Open Source Network Simulator</h1>

  [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg?style=flat-square)](https://www.gnu.org/licenses/agpl-3.0)
  [![Version](https://img.shields.io/badge/version-1.1.0-blue.svg?style=flat-square)](https://github.com/netsim-labs/netsim/releases)
  [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](CONTRIBUTING.md)
  [![Rust](https://img.shields.io/badge/Engine-Rust/WASM-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
  [![TypeScript](https://img.shields.io/badge/UI-TypeScript-blue?style=flat-square&logo=typescript)](https://www.typescriptlang.org/)

  <p>
    <strong>Engineered for High-Performance Network Labs.</strong><br>
    A deterministic simulation engine written in <b>Rust</b> and compiled to <b>WebAssembly</b>.<br>
    Zero virtualization. Zero latency. 100% Browser-native.
  </p>

  [**Live Demo (SaaS)**](https://netsim.dev) • [**Report Bug**](https://github.com/netsim-labs/netsim/issues) • [**Request Feature**](https://github.com/netsim-labs/netsim/issues)
</div>

---

## 🚀 Overview

**NetSim Community Edition** is the open-source core engine behind [NetSim.dev](https://netsim.dev). Starting with **v1.1.0**, the core simulation logic has been migrated from TypeScript to **Rust/WebAssembly** to achieve near-native execution speeds and deterministic protocol behavior.

It provides a high-performance, client-side network simulation environment that runs entirely in your browser, bypassing the need for heavy VMs or backend servers.

<div align="center">
  <img src="docs/images/netsim-dashboard.png" alt="NetSim Dashboard" width="100%" style="border-radius: 8px; border: 1px solid #333;" />
</div>

---

## ✨ Key Features (v1.1.0 WASM Core)

* **🦀 Rust-Powered Engine:** High-performance core handling packet switching and complex routing logic with memory safety.
* **🌐 100% Client-Side:** Runs entirely in your browser using WebAssembly. Zero latency, zero data egress.
* **🔌 Multi-Vendor CLI:** Native support for Cisco (IOS) and Huawei (VRP) command syntaxes.
* **🔒 Privacy First:** Your topologies and configurations never leave your machine.
* **🔋 Built-in Labs:** Ready-to-use scenarios for STP, OSPF, and VLAN practice.
* **🛠️ Modern Stack:** Built with React 18, Rust, and React Flow.

---

## 👀 Visuals

<div align="center">
  <img src="docs/images/netsim-lab-session.png" alt="NetSim Lab Session" width="100%" style="border-radius: 8px; border: 1px solid #333; margin-top: 10px; margin-bottom: 20px;" />
</div>

---

## 📦 Quick Start

Prerequisites: Node.js 18+, Rust toolchain, and `wasm-pack`.

```bash
# 1. Clone the repository
git clone [https://github.com/netsim-labs/netsim.git](https://github.com/netsim-labs/netsim.git)

# 2. Enter the directory
cd netsim-community-v1.1.0

# 3. Build the WASM Core and Install JS dependencies
npm install

# 4. Start the local engine
npm run dev
```

## 🏗️ Architecture

NetSim utilizes a decoupled architecture where a high-speed Rust core manages the network state, exposed to a React frontend via a typed WebAssembly bridge.

| Layer | Technology | Description |
| :--- | :--- | :--- |
| **Core Engine** | Rust + WASM | Logic for OSPF, BGP, STP, and CLI parsing. |
| **State Management** | Zustand | Syncs the WASM state with the UI components. |
| **Visualization** | React Flow | Interactive node-based topology editor. |
| **Interface** | Tailwind CSS | Responsive, dark-themed professional UI. |

## 🤝 Contributing & Governance

We welcome contributions! Please follow these steps:
1.  Read our [Code of Conduct](CODE_OF_CONDUCT.md) to maintain a professional environment.
2.  Fork the repo and create your feature branch.
3.  Ensure your Rust modules pass all tests (`cargo test`).
4.  Open a Pull Request.

## 📄 License

Distributed under the GNU Affero General Public License v3.0 (AGPL-3.0).

*   **Free for personal and educational use.**
*   **Commercial use requires a dedicated license.** Contact `info@netsim.dev`.

<div align="center">
  <p>Made with ❤️ and <b>Data Packets</b> by the <strong>NetSim Labs Team</strong></p>
  <p><small>Copyright © 2026 NetSim Labs. All rights reserved.</small></p>
</div>
