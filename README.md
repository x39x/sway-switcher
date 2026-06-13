# sway focus switcher

A minimal focus switcher for Sway.

## Features

- Tracks focus changes using Sway IPC events
- Switches between the current and previous window
- Works across workspaces

## Usage

Add a custom key binding in your Sway configuration:

```ini
bindsym $mod+Tab nop
```

The program listens for binding events and reacts when the binding command is `nop`.

Start the program in the background:

```sh
# in ~/.config/sway/config
exec sway-switcher

# or manually
sway-switcher &
```

Now pressing `Mod+Tab` will toggle between the current and previously focused window.

## Building

### Compile

```sh
cargo build --release
```

### Install

```sh
cargo install --path .
```

Or copy the binary manually:

```sh
cp target/release/sway-switcher ~/.local/bin/
```
