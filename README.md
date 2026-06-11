# scout

> A local WiFi scanner and connection manager.

`scout` is a single-binary app for scanning nearby WiFi networks,
viewing signal strength, and connecting / disconnecting. Runs on
Windows and Linux with no admin elevation (uses the OS's standard
WiFi API; only writes that require elevation are surfaced with a
clear prompt).

`scout` is part of the [local76](https://github.com/local76/local76)
ecosystem and depends on [`library`](https://github.com/local76/library)
for its widgets and design system.

---

## Features

- **SSID scan.** Lists every visible network with SSID, BSSID, signal
  bars, channel, security type.
- **Connect / disconnect.** Pick a network, enter the PSK, connect.
  Saved networks get a one-tap reconnect.
- **Live signal updates.** Signal bars update at 1-second intervals
  as the user moves.
- **Per-network history.** Last-seen, signal trend (rising / falling
  / stable).
- **Saved profiles.** Stored in the OS's standard credential store
  (Windows: Credential Manager, Linux: NetworkManager secrets).

---

## Install

### Windows
- **Standalone**: download `scout.exe` from the
  [latest release](https://github.com/local76/scout/releases).

### Linux
- **Debian/Ubuntu**: `sudo dpkg -i scout.deb` (downloaded from the
  release page)

Requires `NetworkManager` and `nmcli` on Linux.

---

## Usage

```
scout                      # launch the scanner
scout scan                 # one-shot scan, print SSIDs to stdout
scout connect <ssid>       # connect to a saved network
scout disconnect           # disconnect the active network
scout saved                # list saved profiles
scout --version
scout --help
```

Inside the app:

| Key | Action |
|---|---|
| `↑` / `↓` | Move selection |
| `Enter` | Connect to the selected network (prompts for PSK) |
| `c` | Connect to a saved network |
| `d` | Disconnect the active network |
| `r` | Rescan |
| `q` | Quit |

---

## Configuration

A YAML config file is auto-generated on first run:

- **Windows**: `%APPDATA%\local76\app\scout\config.yaml`
- **Linux**: `~/.config/local76/app/scout/config.yaml`

---

## Build from source

```pwsh
git clone https://github.com/local76/scout.git
cd scout
cargo build --release
```

---

## License

MIT. See [LICENSE.md](LICENSE.md).
