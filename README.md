# Touhou7-remapper

A simple remapper using evdev and uinput crates

## Default keybindings

| Input | Output |
|-------|--------|
| W A S D | ↑ ← ↓ → |
| U | Z (shot) |
| I | X (bomb) |
| Shift | Shift |
| Q | Exit program |

## Installation (Arch Linux)

### Dependencies


Make sure you have Rust installed:
```bash
cargo --version
```
If not:
```bash
sudo pacman -S rust
```

Add your user to the `input` and `uinput` groups:
```bash
sudo usermod -aG input,uinput $USER
```

Enable the `uinput` module:
```bash
sudo modprobe uinput
echo "uinput" | sudo tee /etc/modules-load.d/uinput.conf
```

Log out and back in for group changes to take effect.

### Build

```bash
git clone https://github.com/Alemorii/touhou7-remapper-.git
cd touhou7-remapper-
cargo build --release
```

### Run

```bash
./target/release/touhou-remap
```

Run this in a separate terminal before launching Touhou 7. Press `Q` to exit.
