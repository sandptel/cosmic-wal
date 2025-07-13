# cosmic-wal

A dynamic theme updater for COSMIC Desktop Environment that automatically syncs your desktop theme with pywal-generated color schemes.

## Overview

https://github.com/user-attachments/assets/2c85e2d7-e099-4886-8b77-6c83dd42946a

`cosmic-wal` bridges the gap between pywal's automatic wallpaper color extraction and COSMIC DE's theming system. It monitors pywal's color output and automatically updates your COSMIC theme to match your wallpaper colors, creating a cohesive and dynamic desktop experience.

## Features

- 🎨 **Automatic Theme Syncing**: Monitors pywal color changes and updates COSMIC theme in real-time
- ⚙️ **Configurable Color Mapping**: Customize which pywal colors are used for different theme elements
- 🔄 **Multiple Operation Modes**: One-time refresh or continuous daemon mode
- 📁 **Smart Configuration**: Auto-generates default config if none exists
- 🌓 **Theme Mode Aware**: Works with both dark and light COSMIC themes

## Installation

### Prerequisites

- COSMIC Desktop Environment
- pywal (or wallust/any tool that generates `~/.cache/wal/colors.json` or any equivalent)
- Rust toolchain (for building from source)

### From Source

```bash
git clone https://github.com/yourusername/cosmic-wal.git
cd cosmic-wal
cargo build --release
sudo cp target/release/cosmic-wal /usr/local/bin/
```

## Usage

### One-time Theme Update

Update your COSMIC theme once based on current pywal colors:

```bash
cosmic-wal --refresh
```

### Daemon Mode

Start the daemon to automatically update themes when pywal colors change:

```bash
cosmic-wal --daemon
```

### Help

```bash
cosmic-wal --help
```

## Configuration

cosmic-wal uses a TOML configuration file located at `~/.config/cosmic-wal/config.toml`. If this file doesn't exist, it will be automatically created with default values on first run.

### Default Configuration

```toml
[colors]
accent_color = "color13"
success_color = "color12"
warning_color = "color14"
destructive_color = "color11"
bg_color = "background"
primary_container_color = "color1"
neutral_tint_color = "color9"
text_tint_color = "foreground"
```

### Configuration Options

| Option | Description | Default | Available Values |
|--------|-------------|---------|------------------|
| `accent_color` | Primary accent color for UI elements | `"color13"` | `color0`-`color15`, `background`, `foreground`, `cursor` |
| `success_color` | Color for success states and positive actions | `"color12"` | `color0`-`color15`, `background`, `foreground`, `cursor` |
| `warning_color` | Color for warning states and caution | `"color14"` | `color0`-`color15`, `background`, `foreground`, `cursor` |
| `destructive_color` | Color for destructive actions and errors | `"color11"` | `color0`-`color15`, `background`, `foreground`, `cursor` |
| `bg_color` | Background color for various UI elements | `"background"` | `color0`-`color15`, `background`, `foreground`, `cursor` |
| `primary_container_color` | Container background color | `"color1"` | `color0`-`color15`, `background`, `foreground`, `cursor` |
| `neutral_tint_color` | Neutral tinting for UI elements | `"color9"` | `color0`-`color15`, `background`, `foreground`, `cursor` |
| `text_tint_color` | Text color tinting | `"foreground"` | `color0`-`color15`, `background`, `foreground`, `cursor` |

### Customizing Colors

To customize which pywal colors are used for different theme elements, edit the config file:

```toml
[colors]
# Use a different color for accent
accent_color = "color5"

# Use the wallpaper's background for containers
primary_container_color = "background"

# Disable a color mapping by setting to null (uses fallback)
warning_color = null
```

## Pywal Integration

### Basic Workflow

1. Use pywal to generate colors from your wallpaper:
   ```bash
   wal -i /path/to/your/wallpaper.jpg
   ```

2. Run cosmic-wal to update your COSMIC theme:
   ```bash
   cosmic-wal --refresh
   ```

3. Or start the daemon for automatic updates:
   ```bash
   cosmic-wal --daemon
   ```

### Automatic Startup

To automatically start cosmic-wal when your session begins, add it to your startup applications or create a systemd user service:

#### Systemd User Service

Create `~/.config/systemd/user/cosmic-wal.service`:

```ini
[Unit]
Description=Cosmic WAL Theme Updater
After=graphical-session.target

[Service]
Type=simple
ExecStart=/usr/local/bin/cosmic-wal --daemon
Restart=always
RestartSec=5

[Install]
WantedBy=default.target
```

Enable and start the service:

```bash
systemctl --user enable cosmic-wal.service
systemctl --user start cosmic-wal.service
```

## File Locations

- **Colors Input**: `~/.cache/wal/colors.json` (pywal output)
- **Configuration**: `~/.config/cosmic-wal/config.toml`
- **COSMIC Theme Files**: Managed automatically by COSMIC config system

## Troubleshooting

### Colors Not Updating

1. Ensure pywal has generated colors:
   ```bash
   ls -la ~/.cache/wal/colors.json
   ```

2. Check if the colors file is valid JSON:
   ```bash
   cat ~/.cache/wal/colors.json | jq .
   ```

3. Run cosmic-wal with verbose output to see any errors:
   ```bash
   cosmic-wal --refresh
   ```

### Permission Issues

Ensure cosmic-wal has read access to the pywal colors file and write access to COSMIC config directories.

### Daemon Not Responding

Check if the daemon is running:
```bash
ps aux | grep cosmic-wal
```

Restart the daemon:
```bash
pkill cosmic-wal
cosmic-wal --daemon
```

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Dependencies

- `cosmic-config`: COSMIC configuration management
- `cosmic-theme`: COSMIC theming system  
- `serde`: Serialization/deserialization
- `serde_json`: JSON handling for pywal colors
- `toml`: TOML configuration parsing
- `notify`: File system monitoring
- `tokio`: Async runtime

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

### Development Setup

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [pywal](https://github.com/dylanaraps/pywal) - Automatic color scheme generation
- [COSMIC Desktop](https://github.com/pop-os/cosmic-epoch) - The desktop environment this tool supports
- The Rust community for excellent crates and documentation
