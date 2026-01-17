# NetSim Community Edition

<div align="center">
  <img src="public/logo.png" alt="NetSim Logo" width="120" />
  <h1>The Open Source Network Simulator</h1>

  [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg?style=flat-square)](https://www.gnu.org/licenses/agpl-3.0)
  [![Version](https://img.shields.io/github/v/release/netsim-labs/netsim?style=flat-square&label=version)](https://github.com/netsim-labs/netsim/releases)
  [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](CONTRIBUTING.md)
  [![TypeScript](https://img.shields.io/badge/Written%20in-TypeScript-blue?style=flat-square&logo=typescript)](https://www.typescriptlang.org/)
  [![Open in StackBlitz](https://developer.stackblitz.com/img/open_in_stackblitz.svg)](https://stackblitz.com/github/netsim-labs/netsim)

  <p>
    <strong>Engineered for the Modern Web.</strong><br>
    Browser-based network simulation engine powered by WebAssembly.<br>
    Audit the code. Run locally. Master the CLI.
  </p>

  [**Live Demo (SaaS)**](https://netsim.dev) • [**Report Bug**](https://github.com/netsim-labs/netsim/issues) • [**Request Feature**](https://github.com/netsim-labs/netsim/issues)
</div>

---

## 🚀 Overview

**NetSim Community Edition** is the open-source core engine behind [NetSim.dev](https://netsim.dev). It provides a high-performance, client-side network simulation environment that runs entirely in your browser without requiring backend servers or heavy virtualization.

Designed for students, engineers, and developers who want to understand networking protocols or build their own simulation tools using **React** and **WebAssembly**.

<div align="center">
  <img src="docs/images/netsim-dashboard.png" alt="NetSim Dashboard" width="100%" style="border-radius: 8px; border: 1px solid #333;" />
</div>

## 👀 See It In Action

Experience realistic CLI interactions, real-time topology visualization, and guided labs.

<div align="center">
  <img src="docs/images/netsim-lab-session.png" alt="NetSim Lab Session" width="100%" style="border-radius: 8px; border: 1px solid #333; margin-top: 10px; margin-bottom: 20px;" />
</div>

---

## ✨ Key Features

* **🌐 100% Client-Side:** Runs entirely in your browser using WebAssembly. Zero latency, zero data egress.
* **🔋 Batteries Included:** Comes with **built-in local labs** (STP, OSPF, VLANs) ready to practice immediately.
* **🔌 Multi-Vendor CLI:** Simulation of standard Cisco (IOS) and Huawei (VRP) command syntaxes.
* **🔒 Privacy First:** Your topologies live in your browser's memory or local storage. Nothing is sent to the cloud.
* **💾 Local Persistence:** Save and load your network topologies as JSON files directly from your disk.
* **🛠️ Modern Stack:** Built with React 18, TypeScript, Vite, and React Flow.

## 📦 Quick Start

Prerequisites: Node.js 18+ and npm/pnpm.

```bash
# 1. Clone the repository
git clone https://github.com/netsim-labs/netsim.git

# 2. Enter the directory
cd netsim

# 3. Install dependencies
npm install

# 4. Start the local engine
npm run dev
```

Open http://localhost:5173 to start simulating.

## 🐳 Docker Support

Prefer to run in a container? We've got you covered.

```bash
# Build the image
docker build -t netsim-community .

# Run locally on port 8080
docker run -p 8080:80 netsim-community
```

Open http://localhost:8080 to view.

## 🎮 Usage

### 1. Starting a Lab
Click on the "Labs" icon in the sidebar. You will see a list of built-in scenarios (e.g., "Basic Campus LAN", "OSPF Area 0"). Clicking "Start" will load the topology and instructions instantly.

### 2. The CLI
Click on any device to open the Inspector Panel. Click the >_ terminal icon to open the CLI.
- **Cisco-like:** `en`, `conf t`, `show ip int br`
- **Huawei-like:** `sys`, `display ip int brief`

### 3. Saving Your Work
Since this is the Community Edition, data is not saved to the cloud. Use the Export/Import feature to save your topology as a .json file to your computer.

## 🏗️ Architecture
NetSim uses a headless simulation architecture where the network state is decoupled from the UI.

| Layer | Technology | Description |
| :--- | :--- | :--- |
| **Core Engine** | TypeScript + WASM | Handles packet switching, ARP tables, and STP logic. |
| **State** | Zustand | Global store for device state, cables, and logs. |
| **Visualization** | React Flow | Node-based editor for the topology map. |
| **UI Framework** | Tailwind CSS | Styling and responsiveness. |

## 🤝 Contributing
We welcome contributions from the community! Whether it's fixing a bug in the OSPF logic, adding a new CLI command, or improving the UI.
1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.

## 📄 License

Distributed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.

* Free for personal and educational use.
* Commercial use typically requires a commercial license. Get in touch at **info@netsim.dev**.

---

<div align="center">
  <p>
    Made with ❤️ and <b>Data Packets</b> by the <strong>NetSim Labs Team</strong>
  </p>
  <p>
    <small>Copyright © 2026 NetSim Labs. All rights reserved.</small>
  </p>
</div>
  </p>
</div>
