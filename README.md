# hostedZIP

Multi-project repository by [shirushimori](https://github.com/shirushimori).

## Projects

### [OpenHub](OpenHub/) — Cross-Platform Service Launcher Platform

A modular, high-performance GUI launcher built with **Rust** and **React (TypeScript)**. Handles hosting, updating, and managing third-party payloads, loaders, and cleanup scripts.

- Rust core (Axum + Tokio + Wry)
- React/Vite/TypeScript frontend
- Privilege escalation via raw Win32 API
- [README](OpenHub/README.md)

### [Chams](#) — BlueStacks DLL Installer & Injector

A lightweight DLL installer and injector for BlueStacks emulators, built in Rust with **egui**.

- Downloads and deploys DLLs from GitHub
- DLL injection via `CreateRemoteThread` into `HD-Player.exe`
- Multi-target BlueStacks deployment
- Admin privileges via embedded manifest
- Cross-compiles from Linux to Windows
- [README](#chams-installer) (see below)

---

## Chams Installer

A lightweight DLL installer and injector for BlueStacks emulators, built in Rust with [egui](https://github.com/emilk/egui).

![Platform](https://img.shields.io/badge/platform-Windows-blue)
![Rust](https://img.shields.io/badge/rust-2024-orange)

### Features

- **One-click install** — downloads and deploys DLLs from GitHub
- **DLL injection** — inject `p64.dll` or `n32.DLL` into `HD-Player.exe` via `CreateRemoteThread`
- **Multi-target** — deploys `opengl32.dll` to multiple BlueStacks paths simultaneously
- **Custom targets** — add your own emulator paths
- **Progress bar** — real-time progress during download and installation
- **Auto-launch** — if HD-Player.exe isn't running, asks to launch it before injecting
- **Uninstaller** — generates a `.bat` that cleans everything (files, shortcuts, deployed DLLs)
- **Admin privileges** — requests UAC elevation automatically via embedded manifest
- **No terminal** — runs as a GUI application (no console window)

### Build from Source

**Prerequisites:**
- [Rust](https://rustup.rs/) (edition 2024 / 1.85+)
- `x86_64-pc-windows-gnu` target
- `x86_64-w64-mingw32-windres` (from `mingw-w64`)

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu --release
```

### How It Works

**Install Flow:**
1. Downloads `opengl32.dll`, `n32.DLL`, `p64.dll` from GitHub
2. Saves them to `C:\Users\<user>\dataExfad\DLL\`
3. Copies the installer exe to the same directory
4. Creates desktop and Start Menu shortcuts (with icon)
5. Deploys `opengl32.dll` to selected BlueStacks paths (DLL proxy/sideloading)

**Inject Flow:**
1. Searches for `HD-Player.exe` in known BlueStacks paths
2. Enumerates running processes via `CreateToolhelp32Snapshot`
3. If running: injects via `OpenProcess` -> `VirtualAllocEx` -> `WriteProcessMemory` -> `CreateRemoteThread(LoadLibraryW)`
4. If not running: asks to launch, then injects

### Known BlueStacks Paths

| Path | Target |
|------|--------|
| `C:\Program Files\BlueStacks_nxt` | BlueStacks 5 |
| `C:\Program Files\BlueStacks_msi5` | MSI App Player |
| `C:\Program Files\Bluestacks msi` | BlueStacks MSI |

### Dependencies

| Crate | Purpose |
|-------|---------|
| `eframe` | GUI framework (egui) |
| `ureq` | HTTP client for downloading DLLs |
| `image` | Icon decoding (JPEG -> RGBA) |
| `winres` | Windows resource embedding (manifest, version info) |

### Tech Details

- **DLL Injection**: Uses raw Win32 FFI (`extern "system"`) — no `windows-sys` or `winapi` crate dependency
- **Cross-compilation**: Pure Rust dependencies compile cleanly from Linux to Windows via `x86_64-pc-windows-gnu`
- **Shortcuts**: Created via PowerShell `WScript.Shell` COM object
- **Uninstaller**: Generates a `.bat` script that runs `taskkill`, `rmdir`, `del` with logged output

---

## Author

**shirushimori** — [GitHub](https://github.com/shirushimori)

## License

MIT
