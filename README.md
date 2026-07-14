# Chams Installer

A lightweight DLL installer and injector for BlueStacks emulators, built in Rust with [egui](https://github.com/emilk/egui).

![Platform](https://img.shields.io/badge/platform-Windows-blue)
![Rust](https://img.shields.io/badge/rust-2024-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

- **One-click install** — downloads and deploys DLLs from GitHub
- **DLL injection** — inject `p64.dll` or `n32.DLL` into `HD-Player.exe` via `CreateRemoteThread`
- **Multi-target** — deploys `opengl32.dll` to multiple BlueStacks paths simultaneously
- **Custom targets** — add your own emulator paths
- **Progress bar** — real-time progress during download and installation
- **Auto-launch** — if HD-Player.exe isn't running, asks to launch it before injecting
- **Uninstaller** — generates a `.bat` that cleans everything (files, shortcuts, deployed DLLs)
- **Admin privileges** — requests UAC elevation automatically via embedded manifest
- **No terminal** — runs as a GUI application (no console window)

## Screenshots

> Run the installer from `build/Chams.exe`

## Installation

### From Release

1. Download `Chams.exe` from the [Releases](https://github.com/shirushimori/Chams/releases) page
2. Run as Administrator
3. Select targets and click **Install**

### Build from Source

**Prerequisites:**
- [Rust](https://rustup.rs/) (edition 2024 / 1.85+)
- `x86_64-pc-windows-gnu` target (for cross-compilation from Linux)
- `x86_64-w64-mingw32-windres` (from `mingw-w64`)

```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Build for Windows (from Linux)
cargo build --target x86_64-pc-windows-gnu --release

# Output: target/x86_64-pc-windows-gnu/release/Chams.exe
```

## How It Works

### Install Flow

1. Downloads `opengl32.dll`, `n32.DLL`, `p64.dll` from GitHub
2. Saves them to `C:\Users\<user>\dataExfad\DLL\`
3. Copies the installer exe to the same directory
4. Creates desktop and Start Menu shortcuts (with icon)
5. Deploys `opengl32.dll` to selected BlueStacks paths (DLL proxy/sideloading)

### Inject Flow

1. Searches for `HD-Player.exe` in known BlueStacks paths
2. Enumerates running processes via `CreateToolhelp32Snapshot`
3. If running: injects via `OpenProcess` → `VirtualAllocEx` → `WriteProcessMemory` → `CreateRemoteThread(LoadLibraryW)`
4. If not running: asks to launch, then injects

### Known BlueStacks Paths

| Path | Target |
|------|--------|
| `C:\Program Files\BlueStacks_nxt` | BlueStacks 5 |
| `C:\Program Files\BlueStacks_msi5` | MSI App Player |
| `C:\Program Files\Bluestacks msi` | BlueStacks MSI |

## Project Structure

```
Chams/
├── app.manifest          # Windows manifest (admin privileges)
├── assets/               # App assets
├── build/                # Release builds
│   └── Chams.exe
├── build.rs              # Build script (exe properties, manifest)
├── Cargo.toml            # Dependencies
├── data/
│   ├── Dll/              # Local DLL files (for dev/testing)
│   └── paste_path.txt    # Reference paths
└── src/
    └── main.rs           # Application code
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `eframe` | GUI framework (egui) |
| `ureq` | HTTP client for downloading DLLs |
| `image` | Icon decoding (JPEG → RGBA) |
| `winres` | Windows resource embedding (manifest, version info) |

## Tech Details

- **DLL Injection**: Uses raw Win32 FFI (`extern "system"`) — no `windows-sys` or `winapi` crate dependency
- **Cross-compilation**: Pure Rust dependencies compile cleanly from Linux to Windows via `x86_64-pc-windows-gnu`
- **Shortcuts**: Created via PowerShell `WScript.Shell` COM object
- **Uninstaller**: Generates a `.bat` script that runs `taskkill`, `rmdir`, `del` with logged output

## Author

**shirushimori** — [GitHub](https://github.com/shirushimori)

## License

MIT
