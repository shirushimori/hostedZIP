# hostedZIP

`hostedZIP` is a lightweight asset and configuration host for the **OpenHub** client launcher ecosystem. Specifically, it serves as the live release tracking registry for the **Subh X Cheat** launcher service.

---

## 🛠️ System Architecture

The OpenHub ecosystem consists of:

1. **OpenHub Client Wrapper (Rust)**
   - Powered by [wry](https://github.com/tauri-apps/wry) to render a modern desktop webview interface.
   - Embeds a [Tokio](https://tokio.rs/)/[Axum](https://github.com/tokio-rs/axum) local API server running on `127.0.0.1:3001` to manage background downloads, extraction, and process control.
   - Uses native Windows APIs (`ShellExecuteExW`) for User Account Control (UAC) elevation, allowing it to run installers, launchers, and utilities (like cleaner scripts) as administrator.

2. **Web GUI Frontend (React / TypeScript / Vite)**
   - Custom, responsive user interface communicating with the local Rust API to report download status, setup wizard configurations, and tool listings.

3. **Asset & Registry Host (`hostedZIP`)**
   - This repository acts as the production hosting endpoint for configuration mapping and version telemetry.
   - The launcher reads the raw endpoint of `data/subh/version.txt` to verify update status dynamically.

---

## 📁 Repository Structure

```
.
├── README.md
└── data/
    └── subh/
        └── version.txt   <-- Production Version Registry (e.g., 1.0.2)
```

---

## 🔄 Live Version Checker

The update pipeline relies on:
- **Registry URL**: `https://raw.githubusercontent.com/shirushimori/hostedZIP/refs/heads/main/data/subh/version.txt`
- When client checks updates, it compares the online version with the locally cached `version.txt`. If they do not match, it triggers the update/install payload download.