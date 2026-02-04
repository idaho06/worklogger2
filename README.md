# WorkLogger2

WorkLogger2 is a simple Windows desktop text editor focused on creating daily work log files.

- **Language:** Rust
- **UI:** [native-windows-gui](https://github.com/gabdube/native-windows-gui)
- **Binary name:** `worklogger2.exe`

## Features

- Lightweight text editor UI
- Quick insert helpers (e.g. time)
- Configurable log directory and project name
- Optional desktop shortcut from the installer

## Requirements

- Windows
- Rust toolchain (stable) with Cargo

### Optional (for MSI installer)

- WiX Toolset v3.x installed (provides `candle.exe` and `light.exe` on PATH)
- `cargo-wix`

## Build & run (dev)

```powershell
cargo run
```

## Build (release)

Release builds are optimized for size and strip symbols.

```powershell
cargo build --release
```

The executable will be at:

- `target\release\worklogger2.exe`

## Build MSI installer

This project includes a WiX configuration at `wix/main.wxs`.

1) Install WiX Toolset v3 and ensure `candle`/`light` are available in your PATH.

2) Install cargo-wix (one-time):

```powershell
cargo install cargo-wix
```

3) Build the MSI:

```powershell
cargo wix --nocapture
```

Notes:

- The installer is configured as **per-user** and installs under `%LocalAppData%\WorkLogger2`.
- A Start Menu shortcut is created under `AkinoSoft`. It also registers a hotkey **Ctrl+Alt+L**.
- The Desktop shortcut is an optional feature in the installer UI.

## Project layout

- `src/` – application source
- `resources/` – application resources (icon, rc)
- `wix/` – WiX installer definition

## Contributing

Issues and PRs are welcome.

If you change installer components in `wix/main.wxs`, keep component GUIDs stable.

## License

GPL-2.0-only. See [LICENSE](LICENSE).
